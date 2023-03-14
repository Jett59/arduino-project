use std::{
    error::Error,
    io::{stdin, stdout, Read, Write},
    time::Duration,
};

use serialport::{available_ports, SerialPortType};
use tts::Tts;

fn read_string(reader: &mut impl Read) -> Result<String, Box<dyn Error>> {
    let mut bytes = Vec::new();
    loop {
        let mut buffer = [0; 1];
        reader.read_exact(&mut buffer)?;
        bytes.push(buffer[0]);
        if buffer[0] == b'\n' {
            return Ok(String::from_utf8(bytes)?);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let ports = available_ports()?;
    for (index, port) in ports.iter().enumerate() {
        print!("{}: {}", index + 1, port.port_name);
        if let SerialPortType::UsbPort(port_info) = &port.port_type {
            let product = &port_info.product;
            if let Some(product) = product {
                print!(" ({})", product);
            }
        }
        println!();
    }
    print!(">");
    stdout().lock().flush()?;
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)?;
    let input_string = input_string.trim();
    let mut port: Box<dyn Read> = if input_string.is_empty() {
        Box::new(stdin().lock())
    } else {
        let index = input_string.parse::<usize>()? - 1;
        let port_info = &ports[index];
        println!("Openning {}", port_info.port_name);
        Box::new(
            serialport::new(&port_info.port_name, 57600)
                .timeout(Duration::from_secs(10))
                .open()?,
        )
    };
    println!("Reading...");
    let mut tts_engine = Tts::default()?;
    loop {
        tts_engine.speak(read_string(&mut port)?, true)?;
    }
}
