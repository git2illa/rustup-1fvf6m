#[derive(Debug)]
struct Reactangle {
    width: u32,
    height: u32,
}
fn main() {
    let square = (20,30);

    println!("This reactangle area is {} square pixels.", basic_calc_reactangle_area(square));


    let square = Reactangle{
        width: 20,
        height: 50
    };

    println!("-- {}", struct_calc_area(&square));
}

// tuple way 
fn basic_calc_reactangle_area(dimensions:  (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// struct refactor
fn struct_calc_area(reactangle: &Reactangle) -> u32{

    reactangle.width * reactangle.height

}
