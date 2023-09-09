use std::thread;
use std::time::Duration;
use std::error::Error;

use rppal::gpio::{Gpio, OutputPin, InputPin};

// Pulse width modulation
#[allow(dead_code)]
pub fn making_sounds() -> Result<(), Box<dyn Error>> {
    println!("Running application");
    
    // Setup
    // let gpio: Gpio = Gpio::new()?;
    // let photocell_pin_nr: u8 = 14;
    // let pin_b_nr: u8 = 15;

    // // Init pins
    // let photocell_pin: InputPin = gpio.get(photocell_pin_nr)?.into_input();
    // let mut speaker_pin: OutputPin = gpio.get(pin_b_nr)?.into_output();

    let mut x: u8 = 0;
    loop {

        if x >= 20 {
            break;
        }
        x += 1;
        thread::sleep(Duration::from_millis(200));
    }

    Ok(())
}
