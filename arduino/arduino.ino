#include <NewPing.h>

#define trigPin1 6
#define echoPin1 7

#define trigPin2 8
#define echoPin2 9

#define trigPin3 10
#define echoPin3 11

#define trigPin4 12
#define echoPin4 13

NewPing sensor1(trigPin1, echoPin1);
NewPing sensor2(trigPin2, echoPin2);
NewPing sensor3(trigPin3, echoPin3);
NewPing sensor4(trigPin4, echoPin4);

long doorHeight = 215;  // Or 70 for side of desk.
int UltraSensors[] = { 0, 0, 0, 0 };

void setup() {
  Serial.begin(9600);

  pinMode(trigPin1, OUTPUT);  //setting the sensors as output and input, pretty basic stuff
  pinMode(echoPin1, INPUT);

  pinMode(trigPin2, OUTPUT);
  pinMode(echoPin2, INPUT);

  pinMode(trigPin3, OUTPUT);
  pinMode(echoPin3, INPUT);

  pinMode(trigPin4, OUTPUT);
  pinMode(echoPin4, INPUT);
}

void loop() {
  int height = 0;
  int frameCount = 0;
  boolean validReadingThisFrame;

  do {
    UltraSensors[0] = sensor1.ping_cm();
    delay(5);
    UltraSensors[1] = sensor2.ping_cm();
    delay(5);
    UltraSensors[2] = sensor3.ping_cm();
    delay(5);
    UltraSensors[3] = sensor4.ping_cm();
    delay(5);

    validReadingThisFrame = false;
    int validReadingCount = 0;
    int highestHeightThisFrame = 0;
    for (int i = 0; i < sizeof(UltraSensors) / sizeof(UltraSensors[0]); i++) {
      int reading = UltraSensors[i];
      if (reading > 10 && reading < doorHeight - 20) {
        validReadingThisFrame = true;
        validReadingCount++;
        int heightFromReading = doorHeight - reading;
        if (heightFromReading > highestHeightThisFrame) {
          highestHeightThisFrame = heightFromReading;
        }
      }
    }
    if (validReadingThisFrame) {
      if (highestHeightThisFrame > height) {
        height = highestHeightThisFrame;
      }
      frameCount++;
    }
    if (frameCount > 0 && frameCount % 10 == 0) {
      Serial.print("Waiting: ");
      Serial.print(highestHeightThisFrame);
      Serial.print(", ");
      Serial.println(validReadingCount);
    }
  } while (validReadingThisFrame);
  if (frameCount > 5) {
    Serial.println(height);
  }
  delay(20);
}
