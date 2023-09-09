use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};

// lesson got skipped since the raspberry pi doesn't come with an adc
#[allow(dead_code)]
pub fn traffic_light() -> Result<(), Box<dyn Error>> {
    
    let gpio: Gpio = Gpio::new()?;
    let pins: [(u8, u64); 5] = [(14, 3000), (15, 1000), (18, 3000), (15, 1000), (14, 3000)];
    
    for &pin_nr_delay_tuple in &pins{
        let mut pin: OutputPin = gpio.get(pin_nr_delay_tuple.0)?.into_output();
        pin.set_high();
        thread::sleep(Duration::from_millis(pin_nr_delay_tuple.1));
        pin.set_low();
    }

    Ok(())
}