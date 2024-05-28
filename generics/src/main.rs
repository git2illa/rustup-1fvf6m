struct Point<T,U>{
    x: T,
    y: U
}

struct PointT<T>{
    x: T,
    y: T
}

impl<T> PointT<T>{
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    println!("----");
    let p = PointT{
        x: 5,
        y: 5
    };
    println!("=> {} ", p.x);
    println!("----");
}


fn basic_struct(){
    let integer_float  = Point{
        x: 5,
        y: 2.04
    };
}
