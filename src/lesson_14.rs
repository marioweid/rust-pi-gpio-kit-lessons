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
    let led_pin_nr: u8 = 14;

    let mut led_pin: OutputPin = gpio.get(led_pin_nr)?.into_output();

    let mut x: u8 = 0;
    loop {

        led_pin.toggle();
        thread::sleep(Duration::from_millis(1000));

        if x >= 5 {
            led_pin.set_low();
            break;
        }
        x += 1;
    }

    Ok(())
}
