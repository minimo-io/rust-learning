//!  In Rust, you can specify that a particular bit of code be run whenever a value
//! goes out of scope, and the compiler will insert this code automatically.
//!
//! You specify the code to run when a value goes out of scope by implementing the Drop trait.
//! Requires you to implement 1 method named "drop" that takes a mutable reference to self.
//! Also: The Drop trait is included in the prelude, so we don’t need to bring it into scope.
//! And: Variables are dropped in the reverse order of their creation.
//!
//! Let's do it!
//! Book Chapter: ch15-03-drop.html

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping Custom Smart Pointer with it's data {}", self.data)
    }
}

fn main() {
    let var1: CustomSmartPointer = CustomSmartPointer {
        data: String::from("Pepe"),
    };

    let var2: CustomSmartPointer = CustomSmartPointer {
        data: String::from("Ricky Vainilla"),
    };
    let var3 = CustomSmartPointer {
        data: "minimo".to_string(),
    };

    // Here things go out of scope, and dropped, hence our custom
    // "cleaup code" gets called
    println!("Custom Smart Pointers created");

    // Occasionally, however, you might want to clean up a value early.
    // but Rust doesn’t let you call the Drop trait’s drop method manually.
    // instead you have to call the std::mem::drop function provided by the standard library.
    //
    // This function is in the prelude, so we can just call it `drop(var3)` also.

    std::mem::drop(var3); // force drop it!
    println!("Force dropeed var3 before the end of main");
}
