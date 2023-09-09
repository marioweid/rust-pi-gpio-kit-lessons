use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

#[allow(dead_code)]
pub fn led_blink() -> Result<(), Box<dyn Error>> {
    // Gpio uses BCM pin numbering. BCM GPIO 14 is tied to physical pin 8.
    const GPIO_LED: u8 = 14;

    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin: rppal::gpio::OutputPin = Gpio::new()?.get(GPIO_LED)?.into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    pin.set_high();
    thread::sleep(Duration::from_millis(500));
    pin.set_low();
    Ok(())
}