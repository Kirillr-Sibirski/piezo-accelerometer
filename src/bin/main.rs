#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_println as _;

// ADC
use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 1.0.0
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    const d33 = 295 * 10^(-12); // Piezo constant
    const m = 0.1; // This is the proof mass in kg

    let adc_pin = peripherals.GPIO4;
    let mut adc1_config = AdcConfig::new();
    let mut pin = adc1_config.enable_pin(adc_pin, Attenuation::_11dB); // Need to double check this attenuation level stuff https://esp32.implrust.com/core-concepts/adc/adc-in-esp32.html
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);
    let scaler = 2/3; // Run the 0-5V output through the voltage divider so it can be fed into the microcontroller

    loop {
        let pin_value: u16 = nb::block!(adc1.read_oneshot(&mut pin)).unwrap(); //V, We read the value from the pin here, will need to scale it likely
        esp_println::println!("Raw ADC output:", pin_value); // Print it
        v_out = pin_value / scaler; // Scale the voltage back

        // Need to write some code here
    }
}