use std::thread;
use std::time::Duration;

fn main() {
    generate_workout(28, 3);
}

fn simulate_expensive_calculation(intensity: u32) -> u32 {
    println!("calculation slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_num: u32)  {
    let simulate_result = simulate_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!" , simulate_result);
    } else {
        if random_num == 3 {
            println!("Take a break today!");
        } else {
            println!("Today run for {} minutes", simulate_result);
        }
    }
}
