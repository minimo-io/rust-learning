//! Smart pointers, Deref impl trait
//! By minimo_io

use std::ops::Deref;

fn main() {
    // Regular reference & dereference
    let var1 = 2;
    let var2 = &var1;
    assert_eq!(2, var1);
    assert_eq!(2, *var2);

    println!("var2 = {var2}");

    // Box<T> smart pointer
    // That will store 5 on the heap and the pointer to 5, in the stack
    let i: i32 = 5;
    let c = Box::new(i);
    // Box can be dereferenced
    // because it implements the Deref trait.
    assert_eq!(5, *c);
    println!("c is {c}");

    // Our own MyBox implementation with deferencing (manually implementing the trait)
    let var3 = MyBox(5);
    assert_eq!(5, *var3);

    // Understading deref coercion
    // So, because we implemented the Deref trait on MyBox<T>
    // When the Deref trait is defined for the types involved,
    // Rust will analyze the types and use Deref::deref as many times as necessary
    // to get a reference to match the parameter’s type.
    // The number of times that Deref::deref needs to be inserted is resolved at compile time,
    // so there is no runtime penalty for taking advantage of deref coercion!

    let m = MyBox::new(String::from("Pepe"));
    hello(&m);
    // without deref coercion we would have to do:
    // hello(&(*m)[..]);
}

// Defining our own Box, the infamous MyBox
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
// we implement the dereference trait for MyBox so we can
// dereference values with *
impl<T> Deref for MyBox<T> {
    type Target = T; // associated type (more in Chapter 19)
    fn deref(&self) -> &Self::Target {
        // return the first value of the self tuple struct
        &self.0
    }
}

// Here we study dereference coercion
fn hello(name: &str) {
    println!("Hello {name}");
}
