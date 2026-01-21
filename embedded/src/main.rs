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

    loop {
        let voltage_mv = adc_pin.read()? as f64;
        let v_peak = (voltage_mv - BIAS_VOLTAGE).abs();
        let v_pp = v_peak * 2.0;
        let acceleration = GRADIENT * v_pp + Y_INTERCEPT;

        println!(
            "Voltage: {:.2} mV, Accel: {:.2} g",
            voltage_mv, acceleration
        );

        thread::sleep(Duration::from_millis(500));
    }
    // loop {
    //     let raw = adc_pin.read().unwrap();
    //     println!("RAW ADC: {}", raw);
    //     thread::sleep(Duration::from_millis(500));
    // }
}
