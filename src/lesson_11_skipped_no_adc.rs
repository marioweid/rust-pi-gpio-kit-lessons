use std::thread;
use std::time::Duration;
use std::error::Error;

use rppal::gpio::{Gpio, OutputPin, InputPin};

// Pulse width modulation
#[allow(dead_code)]
pub fn making_sounds() -> Result<(), Box<dyn Error>> {
    println!("Running application");
    
    // Setup
    let gpio: Gpio = Gpio::new()?;
    let photocell_pin_nr: u8 = 14;
    let pin_b_nr: u8 = 15;

    // Init pins
    let photocell_pin: InputPin = gpio.get(photocell_pin_nr)?.into_input();
    let mut speaker_pin: OutputPin = gpio.get(pin_b_nr)?.into_output();

    let mut x: u8 = 0;
    // Somehow partly skipped (no ADC)
    loop {
        let mut readed_value: f64 = 5.0_f64;
        let mut pitch: f64 = 50.0_f64 + readed_value / 4 as f64;

        _ = speaker_pin.set_pwm_frequency(pitch, 1.0 as f64/255.0);
        readed_value += 5_f64;

        if x >= 20 {
            break;
        }
        x += 1;
        thread::sleep(Duration::from_millis(200));
    }

    Ok(())
}
