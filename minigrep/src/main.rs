use std::env;
use std::process;
use cat_minigrep::Config;
use cat_minigrep::run;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else( |err| {
        // cargo run > output.txt
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}

