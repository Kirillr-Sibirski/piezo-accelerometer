clear; close all; clc;

% Test readings, 100mV amplitude on the shaker
ref_v = [180, 166, 142, 124, 107, 95, 84, 75, 67, 60, 53.5, 46, 41]; % mV
act_v = [212, 240, 270, 272, 304, 300, 328, 328, 348, 348, 336, 368, 364]; % mV
freq = [40, 50, 60, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170]; % Hz

% Reference accelerometer (ADXL326)
supply = 5; % V
ref_sensitivity = 57 * (supply/3); % mV/g
ref_a = ref_v / ref_sensitivity;

% Fit: acceleration = m * voltage + b
% We want: ref_a = m * act_v + b
p = polyfit(act_v, ref_a, 1);
m = p(1);  % slope (g/mV)
b = p(2);  % intercept (g)

% Calibration formula
act_a_calibrated = m * act_v + b;

% Calculate R-squared
residuals = ref_a - act_a_calibrated;
SSres = sum(residuals.^2);
SStot = sum((ref_a - mean(ref_a)).^2);
R_squared = 1 - SSres/SStot;

% Display the formula
fprintf('========================================\n');
fprintf('ACCELEROMETER CALIBRATION FORMULA:\n');
fprintf('========================================\n');
fprintf('acceleration (g) = %.8f * voltage (mV) + %.6f\n', m, b);
fprintf('\nR² = %.6f (fit quality)\n', R_squared);
fprintf('========================================\n');

% Plot
figure;
subplot(1,2,1);
hold on;
plot(freq, ref_a, 'r-o', 'DisplayName', 'Reference', 'LineWidth', 1.5);
plot(freq, act_a_calibrated, 'b-o', 'DisplayName', 'Your Accel (calibrated)', 'LineWidth', 1.5);
grid on;
xlabel('Frequency (Hz)');
ylabel('Peak Acceleration (g)');
title('Calibrated Comparison');
legend;
hold off;

subplot(1,2,2);
scatter(act_v, ref_a, 50, 'filled');
hold on;
v_range = linspace(min(act_v), max(act_v), 100);
a_fit = m * v_range + b;
plot(v_range, a_fit, 'r-', 'LineWidth', 2);
grid on;
xlabel('Your Voltage (mV)');
ylabel('Reference Acceleration (g)');
title(sprintf('Calibration Curve\na = %.6f*V + %.4f', m, b));
hold off;