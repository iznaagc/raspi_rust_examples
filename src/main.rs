// mod led_flash_001;
mod led_pwm_002;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("Hello, world!");
    // led_flash_001::blink::blink()
    println!("|||| led_pwm_002 ||||");
    led_pwm_002::pwm::pwm_led()
}
