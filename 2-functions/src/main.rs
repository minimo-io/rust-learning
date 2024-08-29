// Rust is statically typed meaning that it must know the types of all variables at compile time.
// Scalar types represent a single value: char, boolean, integer, floating-point number.

fn main() {
    another_function_baby(5);
}
// this is a function, with an un-used char inside (in simple quotes)
fn another_function_baby(x: i32){
    let some_char: char = '🥇';
    println!("{some_char}");
    println!("{}", x);
}