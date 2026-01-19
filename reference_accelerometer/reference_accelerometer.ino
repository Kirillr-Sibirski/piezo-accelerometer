#include <Wire.h>
#include <Adafruit_Sensor.h>
#include <Adafruit_ADXL345_U.h>

/* Assign a unique ID to this sensor at the same time */
Adafruit_ADXL345_Unified accel = Adafruit_ADXL345_Unified(12345);

#define SAMPLE_RATE 500                               // 500 Hz like an oscilloscope
#define WINDOW_MS 200                                 // 200 ms window
#define BUFFER_SIZE 100  // 500Hz * 200ms = 100 samples

int16_t xBuffer[BUFFER_SIZE];
uint16_t bufIndex = 0;
bool bufferFilled = false;


void displaySensorDetails(void) {
  sensor_t sensor;
  accel.getSensor(&sensor);
  Serial.println("------------------------------------");
  Serial.print("Sensor: ");
  Serial.println(sensor.name);
  Serial.print("Driver Ver: ");
  Serial.println(sensor.version);
  Serial.print("Unique ID: ");
  Serial.println(sensor.sensor_id);
  Serial.print("Max Value: ");
  Serial.print(sensor.max_value);
  Serial.println(" m/s^2");
  Serial.print("Min Value: ");
  Serial.print(sensor.min_value);
  Serial.println(" m/s^2");
  Serial.print("Resolution: ");
  Serial.print(sensor.resolution);
  Serial.println(" m/s^2");
  Serial.println("------------------------------------");
  Serial.println("");
  delay(500);
}

void displayDataRate(void) {
  Serial.print("Data Rate: ");

  switch (accel.getDataRate()) {
    case ADXL345_DATARATE_3200_HZ:
      Serial.print("3200 ");
      break;
    case ADXL345_DATARATE_1600_HZ:
      Serial.print("1600 ");
      break;
    case ADXL345_DATARATE_800_HZ:
      Serial.print("800 ");
      break;
    case ADXL345_DATARATE_400_HZ:
      Serial.print("400 ");
      break;
    case ADXL345_DATARATE_200_HZ:
      Serial.print("200 ");
      break;
    case ADXL345_DATARATE_100_HZ:
      Serial.print("100 ");
      break;
    case ADXL345_DATARATE_50_HZ:
      Serial.print("50 ");
      break;
    case ADXL345_DATARATE_25_HZ:
      Serial.print("25 ");
      break;
    case ADXL345_DATARATE_12_5_HZ:
      Serial.print("12.5 ");
      break;
    case ADXL345_DATARATE_6_25HZ:
      Serial.print("6.25 ");
      break;
    case ADXL345_DATARATE_3_13_HZ:
      Serial.print("3.13 ");
      break;
    case ADXL345_DATARATE_1_56_HZ:
      Serial.print("1.56 ");
      break;
    case ADXL345_DATARATE_0_78_HZ:
      Serial.print("0.78 ");
      break;
    case ADXL345_DATARATE_0_39_HZ:
      Serial.print("0.39 ");
      break;
    case ADXL345_DATARATE_0_20_HZ:
      Serial.print("0.20 ");
      break;
    case ADXL345_DATARATE_0_10_HZ:
      Serial.print("0.10 ");
      break;
    default:
      Serial.print("???? ");
      break;
  }
  Serial.println(" Hz");
}

void displayRange(void) {
  Serial.print("Range: +/- ");

  switch (accel.getRange()) {
    case ADXL345_RANGE_16_G:
      Serial.print("16 ");
      break;
    case ADXL345_RANGE_8_G:
      Serial.print("8 ");
      break;
    case ADXL345_RANGE_4_G:
      Serial.print("4 ");
      break;
    case ADXL345_RANGE_2_G:
      Serial.print("2 ");
      break;
    default:
      Serial.print("?? ");
      break;
  }
  Serial.println(" g");
}

void setup(void) {
  Serial.begin(9600);
  Serial.println("Accelerometer Test");
  Serial.println("");

  /* Initialise the sensor */
  if (!accel.begin()) {
    /* There was a problem detecting the ADXL345 ... check your connections */
    Serial.println("Ooops, no ADXL345 detected ... Check your wiring!");
    while (1)
      ;
  }

  /* Set the range to whatever is appropriate for your project */
  accel.setRange(ADXL345_RANGE_16_G);
  // displaySetRange(ADXL345_RANGE_8_G);
  // displaySetRange(ADXL345_RANGE_4_G);
  // displaySetRange(ADXL345_RANGE_2_G);

  /* Display some basic information on this sensor */
  displaySensorDetails();

  /* Display additional settings (outside the scope of sensor_t) */
  displayDataRate();
  displayRange();
  Serial.println("");
}

float peakX = 0;

void loop() {
  sensors_event_t event;
  accel.getEvent(&event);

  // Convert to milli-m/s² (keep sign)
  int16_t x = (int16_t)(event.acceleration.x * 1000);

  // Store in circular buffer
  xBuffer[bufIndex] = x;
  bufIndex++;
  if (bufIndex >= BUFFER_SIZE) {
    bufIndex = 0;
    bufferFilled = true;
  }

  // Calculate rolling peaks
  int16_t peakPos = 0;
  int16_t peakNeg = 0;
  uint16_t count = bufferFilled ? BUFFER_SIZE : bufIndex;

  for (uint16_t i = 0; i < count; i++) {
    if (xBuffer[i] > peakPos) peakPos = xBuffer[i];
    if (xBuffer[i] < peakNeg) peakNeg = xBuffer[i];
  }

  // Back to float for plotting
  Serial.print("X:");
  Serial.print((float)x / 1000.0);
  Serial.print(",PeakPos:");
  Serial.print((float)peakPos / 1000.0);
  Serial.print(",PeakNeg:");
  Serial.println((float)peakNeg / 1000.0);

  delay(1000 / SAMPLE_RATE);
}
