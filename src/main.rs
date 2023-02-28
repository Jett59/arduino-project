use std::{
    error::Error,
    io::{self, stdin, stdout, Write},
    time::Duration,
};

use btleplug::{
    api::{Central, Manager as _, Peripheral, ScanFilter},
    platform::Manager,
};
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    if adapters.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "No bluetooth!").into());
    }
    for adapter in adapters.iter() {
        println!("Scanning {}", adapter.adapter_info().await?);
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Failed to scan");
        time::sleep(Duration::from_secs(10)).await;
        let peripherals = adapter.peripherals().await?;
        if peripherals.is_empty() {
            println!("Nothing there");
        }
        let mut valid_peripherals = Vec::new();
        for peripheral in peripherals {
            let properties = peripheral.properties().await?.unwrap();
            if let Some(name) = &properties.local_name {
                println!("{}: {name}", valid_peripherals.len() + 1);
                valid_peripherals.push((peripheral, properties));
            }
        }
        print!(">");
        stdout().lock().flush()?;
        let mut answer = String::new();
        stdin().read_line(&mut answer)?;
        let choice = answer.trim().parse::<usize>()? - 1;
        let chosen = &valid_peripherals[choice];
        println!("{:?}", chosen.1.local_name);
        chosen.0.connect().await?;
        println!("Connected");
        let services = chosen.0.discover_services().await?;
        println!("Services: {:#?}", services);
        let characteristics = chosen.0.characteristics();
        println!("Characteristics: {:#?}", characteristics);
        // Write hello world to it.
        let characteristic = &mut characteristics.first().expect("No characteristics");
        loop {
            let read_value = chosen.0.read(characteristic).await?;
            println!("Read value: {:?}", read_value);
        }
        chosen.0.disconnect().await?;
    }
    Ok(())
}
