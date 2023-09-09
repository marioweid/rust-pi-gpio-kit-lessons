use std::error::Error;

mod lesson_1;
mod lesson_2;
mod lesson_3;
// mod lesson_4_skipped_no_adc;
mod lesson_5;
mod lesson_6;
mod lesson_7;
mod lesson_8;
mod lesson_9;
mod lesson_10;
// mod lesson_11_skipped_no_adc;
// mod lesson_11_skipped_no_adc;
// mod lesson_13_skipped_to_much_pain_on_breadboard.rs
mod lesson_14;
// mod lesson_15_skipped_nothing_to_do_here;

fn main() -> () {
    // let _res: Result<(), Box<dyn Error>> = lesson_1::led_blink();
    // let _res: Result<(), Box<dyn Error>> = lesson_2::five_led_blink();
    // let _res: Result<(), Box<dyn Error>> = lesson_3::traffic_light();
    // let _res: Result<(), Box<dyn Error>> = lesson_5::fading();
    // let _res: Result<(), Box<dyn Error>> = lesson_6::button();
    // let _res: Result<(), Box<dyn Error>> = lesson_7::responder();
    // let _res: Result<(), Box<dyn Error>> = lesson_8::active_buzzer();
    // let _res: Result<(), Box<dyn Error>> = lesson_9::passive_buzzer();
    // let _res: Result<(), Box<dyn Error>> = lesson_10::rgb_led();
    let _res: Result<(), Box<dyn Error>> = lesson_14::four_n_tree_five();
    print!("Finished");
}


// let mut line: String = String::new();
// let _ = std::io::stdin().read_line(&mut line);      
// match line.trim() {
//     "q" | "quit" => {
//         println!("Quiting ...");
//         // turn of led gpio
//         red_led.set_low();
//         green_led.set_low();
//         yellow_led.set_low();
        
//         // turn of button gpio
//         // red_btn.set_low();
//         // green_btn.set_low();
//         // yellow_btn.set_low();
//         // reset_btn.set_low();
//         break;
//     }
//     _ => {
//         thread::sleep(Duration::from_millis(500));
//         continue;
//     }
// }