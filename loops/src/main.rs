fn main() {
    basic_loop();
}

fn basic_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
        println!("again! -- {}", result);
}
