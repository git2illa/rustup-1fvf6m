fn main() {
    let mut x = 5;
    println!("Hello, world! -- {}", x) ;
    x = 6;
    println!("Hello, world! {}", x);

    let x = x + 1;
    println!("Hello, world! --  {}", x);

    let x = 2.0;
    println!("Hello, world! --  {}", x);

    let x: f32 = 3.0;
    println!("Hello, world! --  {}", x);

    let x: bool  = false;
    println!("Hello, world! --  {}", x);


    let  tup  = (500, 6.4, 1);
    let (_x,y,_z) = tup;
    println!("Hello, world! --  {}", tup.0);
    println!("Hello, world! --  {}", tup.1);
    println!("Hello, world! --  {}", tup.2);
    println!("Hello, world! --  {}", y);

}
