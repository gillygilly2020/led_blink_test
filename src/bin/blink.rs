use std::{thread::sleep, time::Duration};

use rppal::gpio::Gpio;
// pin used: 18
fn main() -> Result<(), Box<dyn std::error::Error>> {
   let gpio = Gpio::new()?;
   let mut pin = gpio.get(18)?.into_output();


   for _ in 0..=10{
    pin.set_high();
    sleep(Duration::from_millis(500));
    pin.set_low();
    sleep(Duration::from_millis(500));
   }

   Ok(())
}
