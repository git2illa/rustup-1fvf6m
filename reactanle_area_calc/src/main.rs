fn main() {
    let square = (20,30);
    println!("This reactangle area is {} square pixels.", basic_calc_reactangle_area(square));
}

fn basic_calc_reactangle_area(dimensions:  (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
