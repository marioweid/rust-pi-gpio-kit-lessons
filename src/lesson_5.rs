use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};

// Pulse width modulation
#[allow(dead_code)]
pub fn fading() -> Result<(), Box<dyn Error>> {
    
    let gpio: Gpio = Gpio::new()?;
    let pin_dimmed: u8 = 23;
    let pin_normal: u8 = 24;
    let duty_cycle: u64 = 70;
    
    let frequency : u8 = 100; // hz
    let target_ms: u64 = u64::from(1/ frequency);
    let on_time: u64 = target_ms * duty_cycle / 100;
    let off_time: u64 = u64::from(target_ms - on_time);


    loop {
        let mut pin_a: OutputPin = gpio.get(pin_dimmed)?.into_output();
        pin_a.set_high();
        thread::sleep(Duration::from_millis(on_time));
        pin_a.set_low();
        thread::sleep(Duration::from_millis(off_time));

        let mut pin_b: OutputPin = gpio.get(pin_normal)?.into_output();
        pin_b.set_high();
    }
}
