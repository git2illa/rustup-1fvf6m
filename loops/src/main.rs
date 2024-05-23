fn main() {
    basic_loop();
    basic_iter();
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

fn basic_iter() {
    let a = [10, 20 , 30 ,40 ,50];

    for element in a.iter() {
        println!("iterator current -- {}", element);
    }
}
