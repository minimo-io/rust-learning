// Dereferencing is to access the value stored at a memory address.
// Rustaceans use the * operator
fn main() {
    let numbers = [1, 41, 4];
    let number = &numbers[1];

    println!("{}", *number);
    // in this case this will also be valid:
    // println!("{}", number);
    // Because when a pointer is printed, Rust automatically dereferences it 
    // and displays the value it points to

    // Dereferencing a mutable pointer allows you to modify the value stored at the memory address it points to. This is crucial for working with mutable data in Rust.
    println!("-------------");
    let mut number = 5;
    println!("Number is: {}", &number);
    let ptr_to_number = &mut number;
    *ptr_to_number = 10;
    println!("Number is now: {}", number);
}
