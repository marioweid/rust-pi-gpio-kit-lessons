use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};

#[allow(dead_code)]
pub fn five_led_blink() -> Result<(), Box<dyn Error>> {
    
    let gpio: Gpio = Gpio::new()?;
    let pins: [u8; 5] = [14, 15, 18,  23, 24];
    
    for &pin_number in &pins{
        let mut pin: OutputPin = gpio.get(pin_number)?.into_output();
        pin.set_high();
        thread::sleep(Duration::from_millis(500));
        pin.set_low();
    }

    Ok(())
}