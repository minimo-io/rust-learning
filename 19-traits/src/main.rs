// traits are functionality that a type can share with other types
// its share behaviour in an abstract way
// methods we can call on the type
// traits are grounds of these methods for those types

use traits::{NewsArticle, Summary, Tweet};

fn main() {
    println!("Hello, world!");
    let news_1 = NewsArticle{
        headline: String::from("pepepe"),
        location: "Belarus".to_string(),
        author: "minimo".to_string(),
        content: "Hello baby doll".to_string()
    };

    let summary_1 = news_1.summarize(); // trait of our struct 
    println!("{}",summary_1);

    let tweet_1 = Tweet{
        content: "Yeah!".to_string(),
        username:"@minimo_io".to_string(),
        reply:false,
        reteet: false,
    };
    // tweet_1 implements default trait for summarize
    println!("{}", tweet_1.summarize());
}
