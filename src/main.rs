mod led_flash_001;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    led_flash_001::blink::blink()
}
