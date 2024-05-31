use std::thread;
use std::time::Duration;

struct Cacher<T> 
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T>{
        Cacher{
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32{
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }

}

fn main() {
    let simulate_user_intensity = 24;
    let simulate_random_number = 5;
    generate_workout(simulate_user_intensity, simulate_random_number);
}

fn simulate_expensive_calculation(intensity: u32) -> u32 {
    println!("calculation slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_num: u32)  {
    //let simulate_result = simulate_expensive_calculation(intensity);
    let mut  simulate_expensive_closure = Cacher::new(|num: u32| {
        println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!" , simulate_expensive_closure.value(intensity));
        println!("Next, do {} situps!", simulate_expensive_closure.value(intensity));
    } else {
        if random_num == 3 {
            println!("Take a break today!");
        } else {
            println!("Today run for {} minutes", simulate_expensive_closure.value(intensity));
        }
    }
}
