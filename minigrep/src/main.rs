use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args);

    let contents = fs::read_to_string(config.filename)
        .expect("Something wrong when reading the file.");

    println!("Text contents: \n {}", contents);
}

struct Config{
    query: String,
    filename: String
}

impl Config{
    fn new(args: &[String]) -> Config{
        if args.len() < 3 {
            panic!("not enough params!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Config {
            query,
            filename
        }
    }
}
