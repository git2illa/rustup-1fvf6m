fn main() {

    basic_string();
}

fn basic_string(){
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("-- {}", s);
}
