use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args);
    // dbg!(args);

    println!("In file {}", config.query);
    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file.");

    println!("Wiht text: \n{contents}");

    // println!("Searching {query}");


}
struct Config{
    query: String,
    file_path: String
}
impl Config{
    fn new(args: &[String]) -> Config{
        if args.len() < 3{
            panic!("not enough arguments");
        }

        let query:String = args[1].clone();
        let file_path = args[2].clone();
    
        Config{query, file_path}
    }
}
