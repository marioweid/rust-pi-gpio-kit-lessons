use std::error::Error;

use rppal::gpio::{Gpio, OutputPin};

// Pulse width modulation
#[allow(dead_code)]
pub fn button() -> Result<(), Box<dyn Error>> {
    
    let gpio: Gpio = Gpio::new()?;
    let pin: u8 = 23;


    let mut pin: OutputPin = gpio.get(pin)?.into_output();
    pin.set_high();
    Ok(())
}
