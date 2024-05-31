use std::thread;
use std::time::Duration;

fn main() {
    let simulate_user_intensity = 21;
    let simulate_random_number = 3;
    generate_workout(simulate_user_intensity, simulate_random_number);
}

fn simulate_expensive_calculation(intensity: u32) -> u32 {
    println!("calculation slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_num: u32)  {
    let simulate_result = simulate_expensive_calculation(intensity);
    let simulate_expensive_closure = |num: u32| {
        println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!" , simulate_result);
        println!("Next, do {} situps!", simulate_result);
    } else {
        if random_num == 3 {
            println!("Take a break today!");
        } else {
            println!("Today run for {} minutes", simulate_result);
        }
    }
}
