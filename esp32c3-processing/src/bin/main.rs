#![no_std]
#![no_main]

use esp_hal::{
    // analog::adc::{Adc, AdcConfig, Attenuation},
    delay::Delay,
    prelude::*,
};
use esp_idf_hal::adc::config::Config;
use esp_idf_hal::adc::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::sys::adc_atten_t;
use esp_println as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    let peripherals = Peripherals::take()?;

    #[cfg(not(esp32))]
    let mut adc = AdcDriver::new(peripherals.adc1, &Config::new().calibration(true))?;

    const ATTENUATION: adc_atten_t = attenuation::DB_12;

    const D33: f64 = 300.0 * 1e-12; // Piezo constant
    const MASS: f64 = 0.1; // Proof mass in kg
    const VCC: f64 = 5.0; // V
    const CF: f64 = 600e-9; // 600 nF
    const SCALAR: f64 = 2.0 / 5.0; // Voltage divider scaling

    // Attenuation 11db sets input voltage range to 0-3.6V
    #[cfg(not(esp32))]
    let mut adc_pin: esp_idf_hal::adc::AdcChannelDriver<{ ATTENUATION }, _> =
        AdcChannelDriver::new(peripherals.pins.gpio4)?;

    let delay = Delay::new();

    loop {
        let pin_value: u16 = adc.read(&mut adc_pin)?; 
        esp_println::println!("ADC value: {}", pin_value);

        let v_measured: f64 = (pin_value as f64 / 4095.0) * 3.3;
        let v_out: f64 = v_measured / SCALAR;
        let acceleration: f64 = (CF * ((VCC / 2.0) - v_out)) / (D33 * MASS);

        esp_println::println!("{}", acceleration);

        delay.delay_millis(100);
    }
}
