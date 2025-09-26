// use std::{thread::sleep, time::Duration};


// use rppal::pwm::{Pwm, Channel, Polarity};

// fn main() -> Result<(), Box<dyn std::error::Error>> {

//     let pwm = Pwm::with_frequency(
//             Channel::Pwm0,
//             1000.0,
//             0.0,
//             Polarity::Normal,
//             true
//         )?;
//         // for loop to take the led from zeor to bright
//         for i in 0..=100 {
//             let duty = i as f64 / 100.0; // converts i to a float to be used as set duty

//             /* 
//                 set duty cycle takes a float btween 0.0 and 1.0 to determine what percent of the time it is "on"
//              */
//             pwm.set_duty_cycle(duty)?;
//             sleep(Duration::from_millis(50));
//         }

//         // for loop to take led from bright to zero
//         for i in (0..=100).rev() {
//             let duty = i as f64 / 100.0;

//             pwm.set_duty_cycle(duty)?;
//             sleep(Duration::from_millis(50));
//         }
//     Ok(())
// }


use std::{thread::sleep, time::Duration};
use rppal::gpio::Gpio;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gpio = Gpio::new()?;
    let mut pin = gpio.get(18)?.into_output();

    let duty = 30; // brightness percentage (0–100)
    loop {
        // One PWM cycle (10 ms total = 100 Hz)
        for step in 0..100 {
            if step < duty {
                pin.set_high();
            } else {
                pin.set_low();
            }
            sleep(Duration::from_micros(100)); // 100 µs step
        }
    }
}

