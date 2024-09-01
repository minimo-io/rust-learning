// Enums are useful when we want to enummerate all posible variants
// an example with the 2 current versions of IP addresses
// This will now be new variables types in Rust

// Rather than putting eumus into structs to define values/data,
// we can define our enum to expect them with eg: (String)
#[derive(Debug)]
enum IpAddrKind{
    V4 (String),
    V6 (String)
}
// an enum with all kinds of data, really!
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// as with structs we can define mehtods for enums
impl Message {
    fn call (&self){
        println!("Some message");
    }
}
// The option type encodes is used in a scenario in which a value could be something or nothing.
// Null is the value that says there is no value
// in those languages, variables can be null or non-null.
// Rust does not have null
// but has an enum that fulfills this purpose.
// This enum is Option<T>
// <T> is a generic type parameter

fn main() {
    // the name of each enum variant that we define also becomes a function that constructs an instance of the enum
    // you can put any kind of data inside an enum variant
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    dbg!(home);

    // one case of Option<T> enum
    let no_number: Option<i32> = None;
    dbg!(no_number);

    // Option<T> values can not make operations of T
    // Eg:
    let some_number: i8 = 23;
    let some_option_number: Option<i8> = Some(4);
    // >>>> let sum = some_number + some_option_number; // <<< will return an erro
    // we should convert Option<i8> to i8
    // So, in order to have a value that CAN be null, you have to explicitly opt-in by
    // making it Option<T>
    // then, to be able to use the value you must handle the case when its null.
    // for this we will use the match expression 

}
