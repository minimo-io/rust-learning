//! Reference counting Smart Pointer, Rc<T>
//! ---------------------------------------
//! Using Rc<T> allows a single value to have multiple owners, for reading only.
//!
//! We use the Rc<T> type when we want to allocate some data on the heap for multiple parts
//! of our program to read and we can’t determine at compile time which part will finish
//! using the data last.
//!
//! You have to enable multiple ownership explicitly by using the Rust type Rc<T>,
//! which is an abbreviation for reference counting.
//!
//! ALSO: Note that Rc<T> is only for use in single-threaded scenarios.

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    // We don’t have to call a function to decrease the reference count like we have to
    // call Rc::clone to increase the reference count: the implementation of the Drop trait
    // decreases the reference count automatically when an Rc<T> value goes out of scope.

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // What we can’t see in this example is that when b and then a go out of scope
    // at the end of main, the count is then 0, and the Rc<List> is cleaned up completely.

    // A simpler example

    let my_string = Rc::new(String::from("Ricky Vainilla"));
    let clone1 = Rc::clone(&my_string);
    println!("Reference couting: {}", Rc::strong_count(&my_string));
    let clone2 = Rc::clone(&my_string);
    // we could make a deep copy with my_string.clone() but we do not need that costly
    // operation, Rc::clone(&T), only increments the reference count, which is much faster.
    println!("Reference couting: {}", Rc::strong_count(&my_string));
    std::mem::drop(clone2);
    println!(
        "Reference couting after force drop: {}",
        Rc::strong_count(&my_string)
    );
}
