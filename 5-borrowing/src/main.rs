// In Rust, references are the same as "borrowing"
fn main() {
    let mut s1: String = String::from("hello dolly"); // a mutable string
    println!("{s1}");
    mut_my_borrowed_stuff(&mut s1); // passed as a mutable reference
    // let r2 = &mut s1; // it is not allowed to have 2 mutable references to a value
    // so we still have access
    println!("{s1}");

    // --------------

    let mut s2 = String::from("yeah?");
    let s3 = &s2;
    let s4 = &s2;
    println!("{s3} and {s4}");
    let s5 = &mut s2;
    println!("s5: {s5}");
    
}

fn mut_my_borrowed_stuff(some_string: &mut String ){ // accepts a mutable reference
    some_string.push_str(", daddy!");
}

// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.

