fn main() {
    basic_stack_moved();
    basic_stack_moved_prove_it();

}

fn basic_stack_moved() {
    let s1 = String::from("hello");
    //let s2 = s1;
    // compile errer s1, moved
    println!("-- {}", s1);
}

fn basic_stack_moved_prove_it() {
     let s1 = String::from("hello");
     let s2 = s1.clone();    // compile errer s1, moved
    //println!("-- {}", s1);
    println!("-- {}", s2);
}
