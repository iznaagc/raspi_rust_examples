use rppal::gpio::Gpio;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;

const GPIO_LED: u8 = 17; // GPIOピン番号
const MAX_DUTY_CYCLE: u8 = 100; // デューティ比の最大値
const PWM_FREQUENCY_HZ: u64 = 100; // PWM周波数（Hz）

pub fn pwm_led() -> Result<(), Box<dyn std::error::Error>> {
  let running = Arc::new(AtomicBool::new(true));
  let r = running.clone();

  // Ctrl+Cハンドラを設定
  ctrlc::set_handler(move || {
      r.store(false, Ordering::SeqCst);
  })?;

  let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

  let pwm_period = Duration::from_secs_f64(1.0 / PWM_FREQUENCY_HZ as f64);

  while running.load(Ordering::SeqCst) {
      // 明るさを徐々に上げる
      for duty_cycle in 0..=MAX_DUTY_CYCLE {
          let on_time = pwm_period.mul_f64(duty_cycle as f64 / MAX_DUTY_CYCLE as f64);
          let off_time = pwm_period - on_time;

          pin.set_high();
          thread::sleep(on_time);
          pin.set_low();
          thread::sleep(off_time);

          if !running.load(Ordering::SeqCst) {
              break;
          }
      }

      // 明るさを徐々に下げる
      for duty_cycle in (0..=MAX_DUTY_CYCLE).rev() {
          let on_time = pwm_period.mul_f64(duty_cycle as f64 / MAX_DUTY_CYCLE as f64);
          let off_time = pwm_period - on_time;

          pin.set_high();
          thread::sleep(on_time);
          pin.set_low();
          thread::sleep(off_time);

          if !running.load(Ordering::SeqCst) {
              break;
          }
      }
  }

  // LEDを消灯
  pin.set_low();
  Ok(())
}