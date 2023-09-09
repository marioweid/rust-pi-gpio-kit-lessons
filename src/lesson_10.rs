use std::thread;
use std::time::Duration;
use std::error::Error;
use rand::Rng;

use rppal::gpio::{Gpio, OutputPin, InputPin};

fn set_rgb(r: &mut OutputPin, g: &mut OutputPin, b: &mut OutputPin, color: (u8, u8, u8)) {
    _ = r.set_pwm_frequency(100.0_f64, color.0 as f64/255.0);
    _ = g.set_pwm_frequency(100.0_f64, color.1 as f64/255.0);
    _ = b.set_pwm_frequency(100.0_f64, color.2 as f64/255.0);
    
}

// Pulse width modulation
#[allow(dead_code)]
pub fn rgb_led() -> Result<(), Box<dyn Error>> {
    println!("Running application");
    
    // Setup
    let gpio: Gpio = Gpio::new()?;
    let r_led_nr: u8 = 16;
    let g_led_nr: u8 = 20;
    let b_led_nr: u8 = 21;
    let btn_nr: u8 = 14;

    // Init pins
    let mut r_channel: OutputPin = gpio.get(r_led_nr)?.into_output();
    let mut g_channel: OutputPin = gpio.get(g_led_nr)?.into_output();
    let mut b_channel: OutputPin = gpio.get(b_led_nr)?.into_output();
    let random_color_btn: InputPin = gpio.get(btn_nr)?.into_input(); // Random Color Button
    
    // Set random seed
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    let mut x: u8 = 0;
    loop {
        let reset_signal: bool = random_color_btn.is_high();

        if reset_signal{
            let rng_r: u8 = rng.gen_range(0..=255);
            let rng_g: u8 = rng.gen_range(0..=255);
            let rng_b: u8 =  rng.gen_range(0..=255);
            set_rgb(&mut r_channel,&mut g_channel, &mut b_channel, (rng_r, rng_g, rng_b));
            if x >= 20 {
                r_channel.set_low();
                g_channel.set_low();
                b_channel.set_low();
                break;
            }
            x += 1;
        }
        println!("reset signal: {}", reset_signal);
        thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
