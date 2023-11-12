use rppal::gpio::Gpio;
use std::{sync::atomic::{AtomicBool, Ordering}, sync::Arc, thread, time::Duration};

const GPIO_LED: u8 = 17;

pub fn blink() -> Result<(), Box<dyn std::error::Error>> {
  let running = Arc::new(AtomicBool::new(true));
  let r = running.clone();

  ctrlc::set_handler(move || {
    r.store(false, Ordering::SeqCst);
  })?;

  let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

  while running.load(Ordering::SeqCst) {
    pin.set_high();
    thread::sleep(Duration::from_secs(1));
    pin.set_low();
    thread::sleep(Duration::from_secs(1));
  }

  pin.set_low();
  Ok(())
}