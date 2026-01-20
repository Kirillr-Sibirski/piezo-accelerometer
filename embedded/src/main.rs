use esp_idf_hal::adc::attenuation;
use esp_idf_hal::adc::oneshot::{config::AdcChannelConfig, AdcChannelDriver, AdcDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys::{self as sys, *};
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take().unwrap(); // Use unwrap for simplicity during debugging
    println!("=== RUST ADC EXAMPLE STARTED ===");
    println!("GPIO4 connected to sensor");

    let mut adc = AdcDriver::new(peripherals.adc1).unwrap();
    println!("ADC1 initialized");

    let config = AdcChannelConfig {
        attenuation: attenuation::DB_11,
        ..Default::default()
    };

    let mut adc_pin = AdcChannelDriver::new(&mut adc, peripherals.pins.gpio4, &config).unwrap();
    println!("ADC pin (GPIO4) configured - READY TO READ");

    // Calibration setup
    // let mut chars: esp_adc_cal_characteristics_t = unsafe { std::mem::zeroed() };
    // let cal_type = unsafe {
    //     esp_adc_cal_characterize(
    //         sys::adc_unit_t_ADC_UNIT_1,
    //         sys::adc_atten_t_ADC_ATTEN_DB_11,
    //         sys::adc_bits_width_t_ADC_WIDTH_BIT_12,
    //         0,
    //         &mut chars,
    //     )
    // };

    // println!("Calibration type: {:?}", cal_type);

    // loop {
    //     let raw = adc_pin.read().unwrap();
    //     println!("RAW ADC: {}", raw); // Print raw value first for debugging

    //     let v_measured: f64 = unsafe { esp_adc_cal_raw_to_voltage(raw as u32, &chars) } as f64;
    //     println!("VOLTAGE: {:.1} mV", v_measured);

    //     let v_peak: f64 = (v_measured - 1140.0).abs();
    //     let v_pp = v_peak * 2.0;
    //     let acceleration = -0.00976875 * v_pp + 4.023748;

    //     println!("ACCEL: {:.2} g", acceleration);

    //     thread::sleep(Duration::from_millis(500)); // Slower for debugging
    // }
    loop {
        let raw = adc_pin.read().unwrap();
        println!("RAW ADC: {}", raw);
        thread::sleep(Duration::from_millis(500));
    }
}
