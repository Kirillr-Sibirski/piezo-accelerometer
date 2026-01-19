use esp_idf_hal::adc::attenuation;
use esp_idf_hal::adc::calibration::{AdcCalibration, LineCalibration};
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
        calibration: true,
        ..Default::default()
    };

    let mut adc_pin: AdcChannelDriver<'_, _, LineCalibration> =  // Specify LineCalibration type
        AdcChannelDriver::new(&mut adc, peripherals.pins.gpio4, &config)?;

    const D33: f64 = 300.0 * 1e-12;
    const MASS: f64 = 0.1;
    const VCC: f64 = 5.0; // Partly determines the offset
    const CF: f64 = 600e-9;
    const SCALAR: f64 = 2.0 / 5.0;

    loop {
        // Read calibrated voltage directly (in mV)
        let v_measured_mv: i32 = adc_pin.read_voltage()?;
        let v_measured = (v_measured_mv as f64) / 1000.0; // Convert mV to V

        let v_out = v_measured / SCALAR;
        let acceleration = (CF * ((VCC / 2.0) - v_out)) / (D33 * MASS);

        println!("{}", acceleration);

        thread::sleep(Duration::from_millis(100));
    }
}
