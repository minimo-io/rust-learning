// match compare a value against patters 
// and then execute code based on those patter matches.

enum Coin{
    Penny,
    Nickle,
    Dime, 
    Quarter
}
fn value_in_cents(coin: Coin) -> u8{
    // the difference with a if statement
    // that match can evaluate to other types than bool (case with the if statement)
    // match compares each value sequentially
    // the code associated with each arm (here: [1, 5, 10, 25]) can be any expression
    // and the result of that expression is returned
    match coin{
        Coin::Penny => {
            println!("So poor...");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}
// match patters must handle all possibilities
// else the code won't compile
// if the "arm" None => None was not present
// Rust wont compile and throw an error: error[E0004]: non-exhaustive patterns: `None` not covered
fn plus_one(x: Option<i32>) ->Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i + 1)
    }
}
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(six);
}
