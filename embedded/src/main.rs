use esp_idf_hal::adc::attenuation;
use esp_idf_hal::adc::oneshot::{config::AdcChannelConfig, AdcChannelDriver, AdcDriver};
use esp_idf_hal::peripherals::Peripherals;
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take()?;

    let mut adc = AdcDriver::new(peripherals.adc1)?;

    let config = AdcChannelConfig {
        attenuation: attenuation::DB_11,
        ..Default::default()
    };

    let mut adc_pin = AdcChannelDriver::new(&mut adc, peripherals.pins.gpio3, &config)?;

    const D33: f64 = 300.0 * 1e-12;
    const MASS: f64 = 0.1;
    const VCC: f64 = 5.0;
    const CF: f64 = 600e-9;
    const SCALAR: f64 = 2.0 / 5.0;

    loop {
        // Use adc_pin.read() instead of adc.read(&mut adc_pin)
        let pin_value = adc_pin.read()?;
        let v_measured = (pin_value as f64 / 4095.0) * 3.3;
        let v_out = v_measured / SCALAR;
        let acceleration = (CF * ((VCC / 2.0) - v_out)) / (D33 * MASS);

        println!("{}", acceleration);

        thread::sleep(Duration::from_millis(100));
    }
}
