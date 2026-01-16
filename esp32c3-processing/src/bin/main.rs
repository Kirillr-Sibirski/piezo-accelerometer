#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    clock::CpuClock,
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main, peripherals,
};
use esp_println as _;

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

    const D33: f64 = 300.0 * 1e-12; // Piezo constant (use scientific notation)
    const MASS: f64 = 0.1; // Proof mass in kg
    const VCC: f64 = 5.0; // V
    const CF: f64 = 600e-12; // 600pF
    const SCALAR: f64 = 2.0 / 5.0; // Voltage divider scalin

    let adc_pin = peripherals.GPIO4;
    let mut adc1_config = AdcConfig::new();
    let mut pin = adc1_config.enable_pin(adc_pin, Attenuation::_11dB); // Need to double check this attenuation level stuff https://esp32.implrust.com/core-concepts/adc/adc-in-esp32.html
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);

    let delay = Delay::new(&esp_hal::clock::Clocks::get());

    loop {
        let pin_value: u16 = nb::block!(adc1.read_oneshot(&mut pin)).unwrap(); //V, We read the value from the pin here, will need to scale it likely
        let v_measured = (pin_value as f64 / 4095.0) * 3.3;

        let v_out = v_measured / SCALAR; // Scale the voltage back
        let acceleration = (CF * ((VCC / 2.0) - v_out)) / (D33 * MASS); // The actual acceleration calculation
        esp_println::println!("{}", acceleration); // Print the raw acceleration data later to be proccessed by the Python script
        // Some debugging code below
        // esp_println::println!(
        //     "Raw ADC: {}, V_measured: {:.3}V, V_out: {:.3}V, Accel: {:.2}",
        //     pin_value,
        //     v_measured,
        //     v_out,
        //     acceleration
        // );

        delay.delay_millis(100); // Sample every 100ms
    }
}
