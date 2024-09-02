// Vectors can hold only one type of values Vec<T>
// This data structure puts all the values next to each other in memory (heap)
fn main() {
    let mut v:Vec<i32> = Vec::new();
    
    // when we create a Vector with values Rust will infer the type
    // so we don't need to anotate them
    let v2 = vec![1, 2, 3]; // a vec! macro that will create a Vec<i32> vector
    // lets interate
    for i in &v2{
        println!("{i}");
    }
    dbg!("{}", v2);


    // iterating to through a mut vector and changing its value
    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        *i += 50; 
        // we use the * operator (dereference)
        // to get the value i before we add 50
    }
    dbg!("{}", v3);

    // we can add elements to a vector with .push method
    v.push(7);
    dbg!("{}", &v);

    // there are two ways of referencing a value inside a vector
    // via reference to the element in the index
    let some_val: &i32 = &v[0];
    println!("First index value is: {some_val}");
    // or using the get method
    let third: Option<&i32> = v.get(0);
    // this get method will return an Option<&T> that we can use with match
    match third{
        Some(third) => println!("The first element is {third}"),
        None => println!("There is no first element")
    }

    // we use the first method i we WANT the program to panic when
    // there index does not exists
    // and the second method when that can happend occasionally as part
    // of the normal behaviour of our program 

    // vectors can hold only one type
    // but that type can be an enum 
    // that defines different types
    enum SpreadSheetCell{
        Text(String),
        Float(f64),
        Int(i32)
    }
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(32.42),
        SpreadSheetCell::Text(String::from("pepe"))
    ];
    dbg!("{row}");
}
