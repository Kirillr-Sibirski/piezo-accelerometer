clear; clc; close all;

% Constants
Cc = 100 * 10^(-9);
Cp = 450 * 10^(-12);
Vcc = 5; % V

% Variables
Cf = 0.6*10^(-9); % Lower freq, determines output voltage 0.6nF
Rf = 25*10^6; % Lower freq

Ri = 15*10^3; % Upper freq 15k ohm

% Calculations
fL = 1/(2*pi*Rf*Cf);

fH = 1/(2*pi*Ri*(Cp+Cc));

fprintf("The lowest frequency is %.2f Hz and the highest is %.2f Hz \n", fL, fH);

% Calculate the output voltage
d33 = 295 * 10^(-12); % piezoelectric constant (need to actually characterize it)
m = 0.1; % proof mass, kg
max_expected_a = 5*9.81; % 5g 
qp = d33 * m * max_expected_a; % max expected charge
V0 = -(qp/Cf) + (Vcc/2); % voltage floor

% Display the calculated output voltage
fprintf("The calculated output voltage at %.2f m/s^2 (max acceleration) is %.2f V\n", max_expected_a, V0);

reference_a = 2*9.81; % equal to flipping the accelerometer upside down - 2g
qp_ref = d33 * m * reference_a; % max expected charge
V0_ref = -(qp_ref/Cf) + (Vcc/2); % voltage floor

% Display the calculated output voltage
fprintf("The calculated output voltage at %.2f m/s^2 (2g) is %.2f V\n", reference_a, V0_ref);

% Reverse d33 calculator
acc = 5*9.81; % m/s^2
v_out = 0.09; % V
qp_rev = -Cf*(v_out-(Vcc/2));
d33_rev = qp_rev/(m*acc);
fprintf("Reverse calculated d33 is %.2f * 10^(-12) m/V", d33_rev/(10^(-12)));

% Can tweak the params above based on this new d33
