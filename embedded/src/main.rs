use esp_idf_hal::adc::attenuation;
use esp_idf_hal::adc::oneshot::{config::AdcChannelConfig, AdcChannelDriver, AdcDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys::{self as sys, *};
use std::thread;
use std::time::Duration; // Import for raw C APIs

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take()?;

    let mut adc = AdcDriver::new(peripherals.adc1)?;

    let config = AdcChannelConfig {
        attenuation: attenuation::DB_11,
        ..Default::default()
    };

    let mut adc_pin = AdcChannelDriver::new(&mut adc, peripherals.pins.gpio4, &config)?;

    // Linear fit graph
    const gradient: f64 = -0.00976875;
    const y-intercept: f64 = 4.023748;

    // Set up calibration characteristics (done once outside the loop)
    let mut chars: esp_adc_cal_characteristics_t = unsafe { std::mem::zeroed() };
    let cal_type = unsafe {
        esp_adc_cal_characterize(
            sys::adc_unit_t_ADC_UNIT_1,
            sys::adc_atten_t_ADC_ATTEN_DB_11,
            sys::adc_bits_width_t_ADC_WIDTH_BIT_12, // Assuming 12-bit resolution (common for ESP32)
            0, // Use eFuse if available, else default Vref (1100mV)
            &mut chars,
        )
    };

    // Optional: Check if calibration is supported/useful
    if cal_type == sys::esp_adc_cal_value_t_ESP_ADC_CAL_VAL_NOT_SUPPORTED {
        println!("Warning: ADC calibration not supported on this chip; using raw estimates.");
        // You could fall back to manual scaling here if needed
    }

    loop {
        let raw = adc_pin.read()?;

        // Convert raw to calibrated voltage (in mV)
        let v_measured = unsafe { esp_adc_cal_raw_to_voltage(raw as u32, &chars) };

        let v_out = v_measured / SCALAR; // Scale it by the voltage divider factor
        let acceleration = gradient*v_out+y-intercept;

        println!("{:.2}", acceleration);

        thread::sleep(Duration::from_millis(100));
    }
}
