use std::thread;
use std::time::Duration;
use std::error::Error;

use rppal::gpio::{Gpio, OutputPin, InputPin};

fn toggle_led(led: &mut OutputPin) {
    if led.is_set_low() {
        led.set_high();
    }
    else{
        led.set_low();
    }
}
// Pulse width modulation
#[allow(dead_code)]
pub fn responder() -> Result<(), Box<dyn Error>> {
    
    // Setup
    let gpio: Gpio = Gpio::new()?;
    let pin_led_1_nr: u8 = 17;
    let pin_led_2_nr: u8 = 22;
    let pin_led_3_nr: u8 = 27;
    
    let pin_btn_1_nr: u8 = 6;
    let pin_btn_2_nr: u8 = 13;
    let pin_btn_3_nr: u8 = 19;
    let pin_btn_4_nr: u8 = 26;


    let mut red_led: OutputPin = gpio.get(pin_led_1_nr)?.into_output(); // red
    let mut green_led: OutputPin = gpio.get(pin_led_2_nr)?.into_output(); // greed
    let mut yellow_led: OutputPin = gpio.get(pin_led_3_nr)?.into_output(); // yellow

    let red_btn: InputPin = gpio.get(pin_btn_1_nr)?.into_input(); // Input Red 
    let green_btn: InputPin = gpio.get(pin_btn_2_nr)?.into_input(); // Input Red 
    let yellow_btn: InputPin = gpio.get(pin_btn_3_nr)?.into_input(); // Input Red 
    let reset_btn: InputPin = gpio.get(pin_btn_4_nr)?.into_input(); // Reset


    println!("Running application");
    let mut x: u8 = 0;
    loop {
        let red_signal: bool = red_btn.is_high();
        let green_signal: bool = green_btn.is_high();
        let yellow_signal: bool = yellow_btn.is_high();
        let reset_signal: bool = reset_btn.is_high();

        if red_signal {
            toggle_led(& mut red_led);
        }
        
        if green_signal{
            toggle_led(& mut green_led);
        }
        
        if yellow_signal{
            toggle_led(& mut yellow_led);
            
        }
        
        if reset_signal{
            toggle_led(& mut red_led);
            toggle_led(& mut green_led);
            toggle_led(& mut yellow_led);
            if x >= 10 {
                red_led.set_low();
                green_led.set_low();
                yellow_led.set_low();
                break;
            }
            x += 1;
        }

        thread::sleep(Duration::from_millis(200));
    }

    Ok(())
}
