use std::thread;
use std::time::Duration;

fn main() {
    todo!();
}

fn basic_calc(intensity: u32) -> u32 {
    println!("calculation slowly...");
    thread::sleep(Duration.from_sec(2));
    intensity
}
