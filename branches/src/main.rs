fn main() {
    bacic_if_fn();
    if_statement_in_let_statement();
}

fn bacic_if_fn()  {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else{
        println!("condition was false");
    }
}

fn if_statement_in_let_statement()  {
    let condition = true;
    let number = if condition {
        5
    } else{
        6
    };
    
    println!("The value of number is : {}", number);
}
