//! Let's talk about the newtype pattern to implement external traits on external types
//!
//! So normally we can only define a trait if EITHER the trait or the type are local
//! to our crate.
//!
//! Get get around that rule we can use the 'newtype pattern', a concept coined
//! at Haskell.
//!
//! In this example from the book (https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types)
//! we will implement a the Display trait on Vec<T> (both defined outside our crate)
//! which The Orphan Rule do not allow.
//!
//! Here is how

use std::fmt::{self, Formatter};

// We create the wrapped Vec<String>, this is a tuple struct
// so we could access it with self.0 (the vector) later in the impl
struct Wrapper(Vec<String>);

// Then we implement the fmt::Display trait for the Wrapper
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // And we use the vec value inside of it
        write!(f, "[{}]", self.0.join(","))
    }
}

fn main() {
    let v = Wrapper(vec![String::from("hello"), String::from("moon")]);
    println!("w = {v}");
    // The above will print `w = [hello,moon]`
}

// The downside of this technique is that Wrapper is a new type
// so it doesn't have the methods of the value its holding (Vec<T> in this case)
//
// You can find a solution for this here (hint, Deref trait):
// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
