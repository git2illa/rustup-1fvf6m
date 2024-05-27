enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main() {
    println!("------------");
    value_in_cents(Coin::Penny);
    println!("------------");
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("C'est un 1 penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
