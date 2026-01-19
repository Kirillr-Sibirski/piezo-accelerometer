clear; close all; clc;

voltage_mV = 180; %mV

supply = 5; % V
sens = 57 * (supply/3); % mV/g
acceleration_g = voltage_mV / sens;

fprintf('%.1f mV = %.3f g\n', voltage_mV, acceleration_g);