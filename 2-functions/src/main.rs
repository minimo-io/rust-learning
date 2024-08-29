// Rust is statically typed meaning that it must know the types of all variables at compile time.
// Scalar types represent a single value: char, boolean, integer, floating-point number.

fn main() {
    another_function_baby(5);
}
// this is a function, with an un-used char inside (in simple quotes)
fn another_function_baby(x: i32){
    let some_char: char = '🥇';
    // Only snake cas vars
    let _some_other_char: String;
    println!("{some_char}");
    println!("{}", x);
    expressions_do_not_include_ending_colons();
    let five = five();
    println!("{five}");
    let plus_8 = plus_eight(250);
    println!("{}", plus_8);
    let condition: bool = true;
    let number = if condition { 10 }else{ 20 };
    println!("{}", number);
    // Rust will not automatically try to convert non-Boolean types to a Boolean.
    // if number {
        // println!("{}", number);
    // }
}

fn expressions_do_not_include_ending_colons(){
    // this is an expression (because it returns a value)
    // Note: x + 2 has no ; if it had it would be a statement, not an expression
    // and thus would NOT return a value
    let y = {
        let x = 3;
        x + 20
    };
    println!("{}", y);
}

fn five()->u8{
    5
}
fn plus_eight(x: i32)->i32{
    x + 8
}