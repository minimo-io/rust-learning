// our trait definition
pub trait Summary {
    fn summarize(&self) -> String {
        // this is not required but we can add default behaviour for the trait
        String::from("(Read more...)")
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub reteet: bool,
}

// will get default behaviour for the trait
impl Summary for Tweet {}
