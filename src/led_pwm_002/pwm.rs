use rppal::gpio::Gpio;
use std::{sync::atomic::{AtomicBool, Ordering}, sync::Arc, thread, time::Duration};

const GPIO_LED: u8 = 17;

pub fn pwm_led() -> Result<(), Box<dyn std::error::Error>> {
  let gpio = GPIO::new()?;
  let mut pin = gpio.get(GPIO_LED)?.into_output();

  let max_duty_cycle = 100; // 最大デューティ比
  let sleep_time = Duration::from_millis(20);

  loop {
    // LEDの明るさを徐々に増やす
    for duty_cycle in 0..=max_duty_cycle {
      pin.set_high();
      sleep(Duration::from_millis((duty_cycle as u64) * 2));
      pin.set_low();
      sleep(Duration::from_millis(((max_duty_cycle - duty_cycle) as u64) * 2));
      sleep(sleep_time);
    }

    // LEDの明るさを徐々に減らす
    for duty_cycle in (0..=max_duty_cycle).rev() {
      pin.set_high();
      sleep(Duration::from_millis((duty_cycle as u64) * 2));
      pin.set_low();
      sleep(Duration::from_millis(((max_duty_cycle - duty_cycle) as u64) * 2));
      sleep(sleep_time);
    }
  }
  Ok(())
}