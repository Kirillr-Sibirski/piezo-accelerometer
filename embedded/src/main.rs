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

    const BIAS_VOLTAGE: f64 = 1140.0;
    const GRADIENT: f64 = -0.00976875;
    const Y_INTERCEPT: f64 = 4.023748;
    const SAMPLE_COUNT: usize = 100;
    const SAMPLE_DELAY_US: u64 = 100; // 10kHz sampling

    loop {
        // Collect multiple samples to find peak
        let mut max_voltage = 0.0f64;
        let mut min_voltage = f64::MAX;

        for _ in 0..SAMPLE_COUNT {
            let voltage_mv = adc_pin.read()? as f64; // Already calibrated to mV
            max_voltage = max_voltage.max(voltage_mv);
            min_voltage = min_voltage.min(voltage_mv);
            thread::sleep(Duration::from_micros(SAMPLE_DELAY_US));
        }

        let avg_voltage = (max_voltage + min_voltage) / 2.0;
        println!(
            "Avg voltage: {:.2} mV (range: {:.2} - {:.2})",
            avg_voltage, min_voltage, max_voltage
        );

        let max_ac = (max_voltage - BIAS_VOLTAGE).abs();
        let min_ac = (min_voltage - BIAS_VOLTAGE).abs();

        // Calculate peak-to-peak
        let v_pp = max_ac.max(min_ac) * 2.0;

        // Calculate acceleration
        let acceleration = GRADIENT * v_pp + Y_INTERCEPT;
        println!(
            "Acceleration: {:.2} g (Vpp: {:.2} mV)\n",
            acceleration, v_pp
        );

        thread::sleep(Duration::from_millis(500));
    }
    // loop {
    //     let raw = adc_pin.read().unwrap();
    //     println!("RAW ADC: {}", raw);
    //     thread::sleep(Duration::from_millis(500));
    // }
}
