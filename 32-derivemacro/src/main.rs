//! Custom derive macro
//!

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// Instead of manuall impl HelloMacro trait
// for Pancakes, we will use a custom derive macro.
#[derive(HelloMacro)]
struct Pancakes;

// Istead of the Derive we could do this manual impl
// But we should do it for each struct or enum
// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, I am Pancake!");
//     }
// }

fn main() {
    Pancakes::hello_macro();
}
