//! # lib.rs minigrep
//!
//! A certain structs with imppl methods
//! for adding the grep functionality written in Rust
//! These type of comments are useful for explaining the purpose of the crate
//! These comments describe the element they are contained into,
//! instead of describing the element after them.
//! So they are useful for describing the general purpose of crates, modules and binaries.

use std::env;
use std::error::Error;
use std::{fs, vec};

pub use self::kinds::Colors;

/// This is public Config struct
/// # An example
/// Remember that we can create Markdown text and also code snippets like the one below.
/// But the snippet will be tested!
/// ```
///     use std::env;
///     use minigrep::Config;
///     let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
///         eprintln!("Problem parsing arguments: {err}");
///         // process::exit(1);
///     });
/// ```
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    // we use the Iterator impl of the train as an argument
    // returned by the env::args() method
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // because the first argument is the name of the program

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }

        // let query: String = args[1].clone();
        // let file_path = args[2].clone();
        //
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // is_ok() returns true if the Result is Ok()
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// Box<dyn Error> means the function will return a type that implements the Error trait
// But we don’t have to specify what particular type the return value will be.
// Giving us the flexibility to return error values that may be of different types in different error cases.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

// Because contents is the argument that contains all of our text and
// we want to return the parts of that text that match,
// we know contents is the argument that should be connected to the return value using the lifetime syntax.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.as_str()))
        .collect()

    // which method to choose ? (the iterator style or the loops style?)
    // Most rust programmers would choose the iterator
    // Iterators are one of Rust’s zero-cost abstractions, so prefer them!
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        // this \ tells Rust to not include a new line character after it
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
        ";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

pub mod kinds {
    pub enum Colors {
        Green,
        Red,
    }
}
