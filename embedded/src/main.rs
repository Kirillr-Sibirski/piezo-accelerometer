use esp_idf_hal::adc::attenuation;
use esp_idf_hal::adc::oneshot::{config::AdcChannelConfig, AdcChannelDriver, AdcDriver};
use esp_idf_hal::peripherals::Peripherals;
// use esp_idf_sys::{self as sys, *};
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap(); // Use unwrap for simplicity during debugging
    let mut adc = AdcDriver::new(peripherals.adc1).unwrap();

    let config = AdcChannelConfig {
        attenuation: attenuation::DB_11,
        ..Default::default()
    };

    let mut adc_pin = AdcChannelDriver::new(&mut adc, peripherals.pins.gpio4, &config).unwrap();

    const SAMPLE_DURATION_MS: u64 = 5000; // Sample for 5 seconds
    const SAMPLE_DELAY_US: u64 = 100; // 10kHz sampling rate

    loop {
        println!("Sampling for 5 seconds...");

        let mut max_raw = 0u16;
        let mut min_raw = u16::MAX;
        let start = std::time::Instant::now();

        // Sample rapidly for 5 seconds to find peaks
        while start.elapsed().as_millis() < SAMPLE_DURATION_MS as u128 {
            let raw = adc_pin.read()?;
            max_raw = max_raw.max(raw);
            min_raw = min_raw.min(raw);
            thread::sleep(Duration::from_micros(SAMPLE_DELAY_US));
        }

        // Convert to millivolts
        let max_mv = (max_raw as f64 / 4095.0) * 3300.0; // 5400
        let min_mv = (min_raw as f64 / 4095.0) * 3300.0;
        let v_pp = max_mv - min_mv;

        // Apply calibration formula: a = 9.6405*e^(0.0002*V) + -50.1697*e^(-0.0480*V)
        let term1 = 9.6405 * (0.0002 * v_pp).exp();
        let term2 = -50.1697 * (-0.0480 * v_pp).exp();
        let acceleration = term1 + term2;

        println!(
            "Min: {:.0}mV, Max: {:.0}mV, Vpp: {:.0}mV, Accel: {:.2}g\n",
            min_mv, max_mv, v_pp, acceleration
        );

        // Small pause before next measurement cycle
        thread::sleep(Duration::from_millis(500));
    }
    // loop {
    //     let raw = adc_pin.read().unwrap();
    //     println!("RAW ADC: {}", raw);
    //     thread::sleep(Duration::from_millis(500));
    // }
}
