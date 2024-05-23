fn main() {
    let _x = five();
    let y = {
        let x = 3;
        x + 1
    };
    println!("Hello, world! --  {}", y);
    println!("Hello, world! --  {}", _x);
}

fn five() -> i32 {
    5
}
