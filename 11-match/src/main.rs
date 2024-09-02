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
// But hey, using match we can also have catch-all patters
// the catch all arm must be last
// Here is an example of a catch call were we won't use the value
// so we use the special _ catch all:

// let dice_roll = 9;
fn lets_roll(dice_roll: u8){
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // unit value; nothing happends if our dice roll it is not 3 or 7 (in our game)
    }
}
fn add_fancy_hat(){}
fn remove_fancy_hat(){}
fn reroll(){}


fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(six);

    // the "if let " control flow is the same a match for cases were we ca
    // only for one match and want to ignore the rest, 
    // _ => ()
    let config_max = Some(3u8);
    if let Some(max) = config_max{
        println!("The maximum is configured to be {max}");
    }    
    // is the same as:
    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _ => (),
    // }
    // the code in the if let does not run, if the value (config_max)
    // doesn't match the pattern Some(max)

    // we can also use and else in the "if let" flow control 
    // that would be the same as the case _ => { //something } in a match 

    if let Some(max) = config_max{
        // something
    }else{
        // same code we would place in a _ => {} match
    }
}   
    
