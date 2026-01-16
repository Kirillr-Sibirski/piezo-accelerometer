# /// script
# requires-python = ">=3.14"
# dependencies = [
#     "matplotlib>=3.10.8",
#     "pyserial>=3.5",
# ]
# ///

import serial
import matplotlib.pyplot as plt

# Configuring the serial port
ser = serial.Serial('/dev/cu.usbmodem1101', 115200) # Might need to change this

values = []

plt.ion()  # interactive mode
fig, ax = plt.subplots()

try:
    while True:
        line = ser.readline().decode().strip()  # read one line from UART
        if line:
            try:
                value = float(line)
                values.append(value)
                if len(values) > 100:  # keep last 100 points
                    values.pop(0)
                ax.clear()
                ax.plot(values)
                # ax.set_ylim(0, 4096)  # ADC full scale
                plt.pause(0.01)
            except ValueError:
                pass  # ignore bad lines
except KeyboardInterrupt:
    print("Exiting...")
finally:
    ser.close()