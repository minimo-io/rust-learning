use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Here with innovate with a closure for the first time in this tutorial
    // these are anonymous functions passed to the unwrap_or_else
    // to handle the error case of our Result
    let config: Config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    // Instead of using a unwrap_or_else, in this case we do not have a value to unwrap (run fn retuns a
    // unit vale instead), so we only care of handling the Error case, that's why we use an "if let"
    if let Err(e) = minigrep::run(config){
        eprintln!("Application error {e}");
        process::exit(1);
    }


}

