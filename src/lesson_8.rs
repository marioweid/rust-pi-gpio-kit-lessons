use std::thread;
use std::time::Duration;
use std::error::Error;

use rppal::gpio::{Gpio, OutputPin};

// Pulse width modulation
#[allow(dead_code)]
pub fn active_buzzer() -> Result<(), Box<dyn Error>> {
    
    // Setup
    let gpio: Gpio = Gpio::new()?;
    let pin_buzzer_nr: u8 = 14;

    let mut buzzer: OutputPin = gpio.get(pin_buzzer_nr)?.into_output();
    buzzer.set_high();

    println!("Running application");
    let mut x: u8 = 0;
    loop {

        if x >= 10 {
            buzzer.set_low();
            break;
        }
        x += 1;
        thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
