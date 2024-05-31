
use std::thread;
use std::time::Duration;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = 4;
        let equare_to_x = |z| z == x; 
        let y = 4;
        assert!(equare_to_x(y));
    }
    #[test]
    fn it_not_works() {
        let x = 4;
        fn equare_to_x(z: i32) -> bool {z == x} 
        let y = 4;
        assert!(equare_to_x(y));
    }


    #[test]
    fn call_with_value() {
        let mut c = Cacher::new(|a| a);
        let c1 =  c.value(1);
        let c2 =  c.value(2);
        assert_eq!(c2, 2);
    }


}

pub struct Cacher<T> 
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    pub fn new(calculation: T) -> Cacher<T>{
        Cacher{
            calculation,
            value: None
        }
    }

    pub fn value(&mut self, arg: u32) -> u32{
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

pub fn simulate_expensive_calculation(intensity: u32) -> u32 {
    println!("calculation slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub fn generate_workout(intensity: u32, random_num: u32)  {
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
