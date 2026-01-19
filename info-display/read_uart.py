# /// script
# requires-python = ">=3.10"
# dependencies = [
#     "esp-idf-monitor>=1.0.0",
#     "matplotlib>=3.8.0",
#     "numpy>=1.24.0",
#     "scipy>=1.10.0",
# ]
# ///

import serial
import matplotlib.pyplot as plt
import numpy as np
from scipy.fft import fft, fftfreq
from collections import deque
import time

# Configure serial port (change as needed; e.g., '/dev/ttyUSB0' on Linux, 'COM3' on Windows)
SERIAL_PORT = "/dev/cu.usbmodem101"
BAUD_RATE = 115200
SAMPLE_INTERVAL = 0.1  # 100ms from ESP loop, in seconds
HISTORY_SECONDS = 5  # Analyze last 5 seconds
MAX_POINTS = 100  # Keep last 100 points for plot

ser = serial.Serial(SERIAL_PORT, BAUD_RATE, timeout=1)

values = deque(maxlen=MAX_POINTS)  # Acceleration values
times = deque(maxlen=MAX_POINTS)  # Timestamps relative to start

plt.ion()  # Interactive mode
fig, ax = plt.subplots()
start_time = time.time()

try:
    while True:
        line = ser.readline().decode().strip()
        if line:
            try:
                value = float(line)
                current_time = time.time() - start_time
                values.append(value)
                times.append(current_time)

                # Plot update
                ax.clear()
                ax.plot(times, values, label="Acceleration")
                ax.set_xlabel("Time (s)")
                ax.set_ylabel(
                    "Acceleration (m/s²)"
                )  # Change to 'g' if dividing by 9.81
                ax.set_title("Real-Time Acceleration")
                ax.legend()
                # ax.set_ylim(-20, 20)  # Adjust based on expected range

                # Analyze last 5 seconds (approx. last N samples)
                num_samples_5s = int(HISTORY_SECONDS / SAMPLE_INTERVAL)
                recent_values = list(values)[-num_samples_5s:]
                if len(recent_values) >= num_samples_5s // 2:  # Need enough data
                    # Peak acceleration (absolute max)
                    peak_accel = max(abs(v) for v in recent_values)

                    # Frequency: Dominant freq via FFT
                    y = np.array(recent_values)
                    N = len(y)
                    yf = fft(y - np.mean(y))  # Remove DC offset
                    xf = fftfreq(N, SAMPLE_INTERVAL)[: N // 2]
                    dominant_freq = xf[np.argmax(np.abs(yf[: N // 2]))]

                    # Annotate on plot
                    ax.text(
                        0.05,
                        0.95,
                        f"Peak Accel (last 5s): {peak_accel:.2f} m/s²\nDominant Freq: {dominant_freq:.2f} Hz",
                        transform=ax.transAxes,
                        va="top",
                        bbox=dict(facecolor="white", alpha=0.5),
                    )

                plt.pause(0.01)
            except ValueError:
                pass  # Ignore bad lines
except KeyboardInterrupt:
    print("Exiting...")
finally:
    ser.close()
