use std::thread;
use std::time::Duration;
use std::error::Error;

use rppal::gpio::{Gpio, OutputPin};

// Pulse width modulation
#[allow(dead_code)]
pub fn four_n_tree_five() -> Result<(), Box<dyn Error>> {
    println!("Running application");
    
    // Setup
    let gpio: Gpio = Gpio::new()?;
    let optocoupler_pin_nr: u8 = 14;

    // Init pins
    let mut optocoupler: OutputPin = gpio.get(optocoupler_pin_nr)?.into_output();

    let mut x: u8 = 0;
    loop {

        optocoupler.set_low();
        thread::sleep(Duration::from_millis(1000));
        optocoupler.set_high();
        thread::sleep(Duration::from_millis(1000));

        if x >= 5 {
            optocoupler.set_low();
            break;
        }
        x += 1;
    }

    Ok(())
}
