% This MATLAB script provides a proof of concept for our microphone-based
% localization strategy of audio impulses.
% Team HEADSUP
% Date: 2/15/2023

num_mics = 8;
speed = 343260; % mm/s
helmet_diameter = 16; % mm
mic_locations = zeros(2, num_mics);
for i=1:num_mics
    mic_locations(:, i) = [helmet_diameter*cos(2*pi*i/num_mics); helmet_diameter*sin(2*pi*i/num_mics)];
end

% impulse_loc = [-3048; -3048];
impulse_loc = [4000; 0];
times = zeros(num_mics, 1);

closest_mic = 1;
min_dist = norm(mic_locations(:, 1) - impulse_loc);
times(1) = min_dist / speed;

for i=2:num_mics
    dist = norm(mic_locations(:, i) - impulse_loc);
    times(i) = dist/speed;
    if dist < min_dist
        min_dist = dist;
        closest_mic = i;
    end
end

times = times - times(closest_mic);
times = times + (1e-6)*randn(num_mics, 1);
plt_mat = [mic_locations, impulse_loc, loc_guess];

% Invoke plane wave assumption and solve for impulse location with LLS
loc_guess = (mic_locations - mic_locations(closest_mic))' \ (times*speed);
loc_guess = loc_guess * -1;
