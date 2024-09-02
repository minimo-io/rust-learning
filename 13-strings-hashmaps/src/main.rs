// Strings are more complex that what they look like
// A String is a wrapper around Vec<u8>
/// This is a documentation line!
fn main() {
    // This is a 4 bytes string. 
    // Each letters takes 1 byte when encoded to utf-8
    let s4  = String::from("Hola");
    // But
    let cy = String::from("Здравствуйте");
    // this cyrillic string takes 24 bytes and not 12.
    // "because each Unicode scalar value in that string takes 2 bytes of storage"
    // https://doc.rust-lang.org/book/ch08-02-strings.html
    // so &cy[0] might not be a valid character on its own.
    // that its why Rust does not let indexing into Strings

    // slicing strings is valid but very prone to error
    // if we try to slice only part of a characters (contained in more that 1 byte)
    // Rust will panic
    let s = &cy[0..4];
    // println!("{}", &s);
    // let ss = &cy[0..1]; // will panic
    // REMEMBER: We can only access (and point to) one byte of memory at a time, not less!

    // to operate with strings its best to be explicit
    // you want characters
    for char in s.chars(){
        println!("{char}");
    }
    //  or bytes?
    for b in s.bytes() {
        println!("{b}");
    }

    // HashMaps are simpler creatures
    // keys and values must have the same types

    // HashMap<K, V>

    // They are not automatically included in the scope in the prelude (as Strings or Vec)
    use std::collections::HashMap; 

    let mut scores = HashMap::new();
    // we use the insert feature to add values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 60);

    // getting values by calling the get method
    // the get method returns Option<&V> or None (if there is no value for the key)
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // here the Option retuned by get is handled with copied() to return an
    // Option<i32> instead of Option<&i32> then unwrap_or to set score to 0
    // in case of None.
    println!("{}", &score);

    // we can interate through them as with vectors
    for (key, value) in &scores{
        println!("{key}: {value}");
    }

    // Hasmaps have a special method called .entry that receives a key
    // that retuns an enum called Entry that represents a value that might or might not exists
    scores.entry(String::from("Purple")).or_insert(100);
    println!("{scores:?}"); // {"Green": 60, "Purple": 100, "Blue": 10}

}
