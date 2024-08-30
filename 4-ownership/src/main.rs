// So finall, some ownership concepts.
// In rust memory is managed by a set of ownership rules, not a garbage collector.
// The stack and the heap are parts of memory available at runtime.
// The Stack: Stores values in the order it gets them and removes them in the opposite order.
// last-in, first-out.
// All data on the stack must have a fixed size.
// THe Heap: is for data unknown at compile time, or data which size need to change.

// Pushing data to the stack is faster, because the allocator does not need to search for a place.
// Accesing data on the heap requires following a pointer.
// The purpose of ownership is managing data on the heap.

// Rules:
// - Each value has an owner
// - There can only be one owner at a time
// - When the owner goes out of scope, the value will be dropped

fn main() {
    // let mut s = String::from("The Magnificent Seven");
    // s.push_str(", The Clash");
    // println!("{s}");

    let s = String::from("hello");
    use_s_on_the_heap(s); // this takes ownership and s is not available anymore (on the heap)
    // println!("{s}"); // this will throw an error

    let x = 5;
    use_x_on_the_stack(x); // for variables on the stack a copy is made
    println!("{x}"); // so this is ok

    // ------------------------------
    let ss = gives_ownership();
    println!("{ss}");

    let ss2 = String::from("String in the heat, mean heap");
    let ss3 = takes_and_give_it_back(ss2);
    println!("{ss3}");
    let (ss4, len) = return_with_tuple(ss3);
    println!("S4: {ss4} and length of {len}");

    // Ownership rule of thumb, assigning a variable (on the heap) to another
    // variables moves it: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
    // Rust also makes it possible for using a value without taking ownership, its called references.
}

fn use_s_on_the_heap(s: String){
    println!("{s}");
}
fn use_x_on_the_stack(x: i32){
    println!("{x}");
}

fn gives_ownership()->String{
    let some_string = String::from("You own me");
    some_string
}
fn takes_and_give_it_back(a_string: String) -> String{
    a_string
}
fn return_with_tuple(some_string: String) -> (String, usize){
    let length = some_string.len();
    (some_string, length)
}