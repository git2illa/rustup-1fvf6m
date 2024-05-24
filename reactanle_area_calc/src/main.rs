#[derive(Debug)]
struct Reactangle {
    width: u32,
    height: u32,
}

impl Reactangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Reactangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let square = (20,30);

    println!("This reactangle area is {} square pixels.", basic_calc_reactangle_area(square));


    let square = Reactangle{
        width: 20,
        height: 50
    };

    println!("-- {:?}", square);
    println!("-- {:#?}", square);

    println!("-- {}", struct_calc_area(&square));
    println!("-- -- {}", square.area());

    let react1 = Reactangle{
        width: 10,
        height: 20
    };

    let react2 = Reactangle{
        width: 15,
        height: 25
    };

    println!("-- {}", square.can_hold(&react1));
    println!("-- {}", square.can_hold(&react2));
}

// tuple way 
fn basic_calc_reactangle_area(dimensions:  (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// struct refactor
fn struct_calc_area(reactangle: &Reactangle) -> u32{

    reactangle.width * reactangle.height

}
