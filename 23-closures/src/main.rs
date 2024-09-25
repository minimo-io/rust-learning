use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    thread::spawn(move || println!("From thread: {list:?}")).join().unwrap();

    // that was kinda too little about closures (anonymous functions that -can- capture variables)
    // in it's scope
    // but let's learn more with iterators
    let mut v1: Vec<String> = Vec::new();
    v1.push(String::from("Stuff_1"));
    v1.push(String::from("Stuff_2"));
    // in rust, interators are lazy (their definition is separate from their usage)
    let v1_iter = v1.iter();
    for val in v1_iter{
        println!("{val}");
    }
    // there are those iterators that consume the iterator (those that call next() method of the Iterator trait). Eg. the sum() method.

    let v2 = vec![1,2,3];
    let v2_iter = v2.iter();
    let sum: u8 = v2_iter.sum();
    println!("{}", sum);

    // and there are ITERATOR ADAPTORS that produce iterators
    // by changing something of the original
    // They are also lazy, meaning they too must be .collect()
    // For example the map .filter() iterator, which receives a CLOSURE as a parameter
    let v3 = vec![
        String::from("pepe"), 
        String::from("martha"), 
        String::from("golang")
    ];

    let v3_filtered:Vec<String> = v3.clone().into_iter().filter(|n| n == "martha").collect();

    dbg!(v3_filtered);
    dbg!(v3); 

}