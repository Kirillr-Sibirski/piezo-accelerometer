clear; close all; clc;

% 700mVpp test data
% ref_v = [1100, 940, 980, 880, 748, 680, 620, 570, 512, 470, 400, 380, 320, 350, 360, 320, 300, 280, 260, 250, 230, 200, 200, 190, 160, 150, 140]; % mV
% act_v = [80, 100, 88, 72, 90, 96, 104, 112, 128, 116, 144, 200, 480, 680, 800, 500, 440, 380, 408, 504, 600, 720, 816, 970, 1160, 1330, 1600]; % mV
% freq = [40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 260, 270, 280, 290, 300]; % Hz

% % 500mVpp test datacx
% ref_v = [1100, 940, 980, 880, 748, 680, 620, 570, 512, 470, 400, 320, 350, 360, 320, 300, 280, 260, 250, 230, 200, 200, 190, 160, 150, 140]; % mV
% act_v =[40, 40, 48, 48, 56, 56, 56, 72, 56, 64, 64, 64, 72, 104, 88, 80, 88, 96, 96, 96, 112, 144, 200, 336, 520, 912]; % mV
% ampl = [40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 180, 190, 200, 210, 220, 230, 240, 250, 260, 270, 280, 290, 300]; % Hz

% 500mVpp test datacx
ref_v = [1100, 940, 980, 880, 748, 680, 620, 570, 512, 470, 400, 320, 350, 360, 320, 300, 280, 260, 250, 230, 200]; % mV
act_v =[40, 40, 48, 48, 56, 56, 56, 72, 56, 64, 64, 64, 72, 104, 88, 80, 88, 96, 96, 96, 112]; % mV
ampl = [40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 180, 190, 200, 210, 220, 230, 240, 250]; % Hz

% Test data
% act_v = [48, 56, 56, 64, 64, 72, 88, 96, 112, 216, 350]; % mV
% ref_v = [464, 540, 616, 680, 744, 800, 840, 900, 912, 940, 980]; % mV
% ampl = [500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500]; % mVpp (optional)

% Reference accelerometer parameters
supply = 5; % V
ref_sensitivity = 57 * (supply / 3); % mV/g
ref_a = ref_v / ref_sensitivity; % Acceleration in g

% Ensure data is column vectors
act_v = act_v(:);
ref_a = ref_a(:);
ampl = ampl(:);

% Sort data by ref_a (acceleration) for better plotting
[ref_a_sorted, idx] = sort(ref_a);
act_v_sorted = act_v(idx);

% List of fit types to try (Voltage over Acceleration)
fit_types = {
    'poly1', ... % Linear: V = m*a + b
    'poly2', ... % Quadratic
    'poly3', ... % Cubic
    'exp1', ...  % V = a*exp(b*a)
    'exp2', ...  % V = a*exp(b*a) + c*exp(d*a)
    'power1', ... % V = a*a^b
    'power2', ... % V = a*a^b + c
    'rat01', ...  % V = a/(1 + b*a)
    'rat11'      % V = (a*a + b)/(a + c)
};

% Structure to store fits and goodness-of-fit
fits = struct();
gof_values = struct();
best_fit = struct('type', '', 'rsquare', -Inf, 'rmse', Inf, 'fitobj', []);

for i = 1:length(fit_types)
    ft_name = fit_types{i};
    try
        ft = fittype(ft_name);
        [fitobj, gof] = fit(ref_a_sorted, act_v_sorted, ft); % Notice: x = ref_a, y = act_v
        
        % Store
        fits.(ft_name) = fitobj;
        gof_values.(ft_name) = gof;
        
        % Update best fit
        if gof.rsquare > best_fit.rsquare || ...
           (abs(gof.rsquare - best_fit.rsquare) < 1e-6 && gof.rmse < best_fit.rmse)
            best_fit.type = ft_name;
            best_fit.rsquare = gof.rsquare;
            best_fit.rmse = gof.rmse;
            best_fit.fitobj = fitobj;
        end
    catch err
        disp(['Error fitting ' ft_name ': ' err.message]);
    end
end

% Display fit results
disp('Fit Results for Voltage over Acceleration:');
field_names = fieldnames(gof_values);
for i = 1:length(field_names)
    ft_name = field_names{i};
    gof = gof_values.(ft_name);
    disp([ft_name ': R² = ' num2str(gof.rsquare, '%.4f') ', RMSE = ' num2str(gof.rmse, '%.4f')]);
    disp(fits.(ft_name));
    disp(' ');
end

disp('Best Fit:');
disp(['Type: ' best_fit.type]);
disp(['R²: ' num2str(best_fit.rsquare, '%.4f')]);
disp(['RMSE: ' num2str(best_fit.rmse, '%.4f')]);
disp(best_fit.fitobj);

% Build equation string
fitobj = best_fit.fitobj;
eq = formula(fitobj);
coeff = coeffvalues(fitobj);
names = coeffnames(fitobj);
for j = 1:length(names)
    eq = strrep(eq, names{j}, num2str(coeff(j), '%.4f'));
end
eq = strrep(eq, 'x', 'a'); % acceleration
eq_str = ['Voltage (mV) = ' eq];
disp('========================================');
disp('BEST FIT CALIBRATION FORMULA (Voltage over Acceleration):');
disp('========================================');
disp(eq_str);
fprintf('\nR² = %.6f\n', best_fit.rsquare);
disp('========================================');

% Predict voltage using best fit
predicted_v = fitobj(ref_a);

% Figure 1: Shaker Amplitude vs Predicted Voltage (optional)
figure(1);
hold on;
plot(ampl, act_v_sorted, 'r-o', 'LineWidth', 1.5, 'MarkerSize', 6, 'DisplayName', 'Measured Voltage');
plot(ampl, predicted_v, 'b-o', 'LineWidth', 1.5, 'MarkerSize', 6, 'DisplayName', [best_fit.type ' Fit']);
grid on;
xlabel('Shaker Amplitude (mVpp)');
ylabel('Voltage (mV)');
title([best_fit.type ' Calibration (Voltage over Acceleration)']);
legend('Location', 'northwest');
hold off;

% Figure 2: Acceleration vs Voltage (Calibration Curve)
a_range = linspace(min(ref_a), max(ref_a), 200);
v_fit = fitobj(a_range);
figure(2);
scatter(ref_a, act_v, 50, 'filled', 'DisplayName', 'Data Points');
hold on;
plot(a_range, v_fit, 'r-', 'LineWidth', 2, 'DisplayName', 'Best Fit');
grid on;
xlabel('Acceleration (g)');
ylabel('Voltage (mV)');
title({[best_fit.type ' Fit'], eq_str, ['R² = ' num2str(best_fit.rsquare, '%.6f')]});
legend('Location', 'northwest');
hold off;