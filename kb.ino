bool lastState2 = HIGH;
bool lastState3 = HIGH;

void setup() {
  Serial.begin(9600);
  pinMode(2, INPUT_PULLUP);
  pinMode(3, INPUT_PULLUP);
}

void loop() {
  bool currentState2 = digitalRead(2);
  bool currentState3 = digitalRead(3);

  if (lastState2 == HIGH && currentState2 == LOW) {
    Serial.println("down");
  }

  if (lastState3 == HIGH && currentState3 == LOW) {
    Serial.println("up");
  }

  lastState2 = currentState2;
  lastState3 = currentState3;

  delay(10);
}
