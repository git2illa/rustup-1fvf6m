//! Minigrep is a demo for beginers to learn rust

use std::fs;
use std::error::Error;
use std::env;


/// # minigrep demo
/// ```rust
/// let query = "duct";
/// let contents = "safe, fast, productive";
/// fn case_sensitive(){
///   println!("{}", query);
///   println!("{}", contents);
/// }
///
///        assert_eq!(
///            vec!["safe, fast, productive."],
///            minigrep::search(query, contents)
///        );
/// // defalut is case_sensitive
/// // use `cargo doc` to view this generated HTML
/// ```

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duck tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive,
Pick three.
Trust me.";
        
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
            );
    }

}

pub fn search<'a>(query: &str,  contents: &'a str) -> Vec<&'a str>{
    contents.lines() 
        .filter(|line| line.contains(query))
        .collect()
}

pub struct Config {
   pub query: String,
   pub filename: String,
   pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str>{
        // env::args的返回值的第一个值是程序本身的名称。为了忽略它，我们必须先调用一次next并忽略返回值
        args.next();


        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename."),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok( Config { query, filename, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("-- {}", line);
    }


    Ok(())
}

pub fn search_case_insensitive<'a>(query: &str,  contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

