fn main() {
    let five = Some(5);
    let six = plus_on(five);
    let none = plus_on(None);
}

fn plus_on(x: Option<i32>) -> Option<i32>{
    match x {
        None => {
            None
        },
        Some(i) => Some(i+1)
    }
}


