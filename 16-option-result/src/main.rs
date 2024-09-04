// We have a function that retuns an Option value
// This structures are for handling cases where it is expected
// to not have a value in some cases
// for example, optional parameters in functions or
// different returned values

// Option enum is used for values that might or might not exist
// Result enum (or type) are used to represent success or failure in an operation
fn divide (num: f64, den:f64) -> Option<f64>{
    if den == 0.0{
        None
    }else{
        Some( num / den)
    }
}
// we need to use a patter match to retrieve this option

use std::fs::{read_to_string, File};
use std::io::{self, Read};

fn main(){ 
    let result = divide(2.0, 5.0);
    match result{
        Some(x) => println!("{}", x),
        None => println!("Cannot divide by 0")

    }

    // then the Result macro is for error handling
    // or when not possible Rust has the panic! macro
    // Errors handled by Result as not serious enough 
    // for the program to panic!

    let greeting_file_result = File::open("src/main.rs"); // File::open has a result type of Result<T, E>
    let greeting_file = match greeting_file_result{
        Ok(file) => file,
        Err(error) => panic!("Could not open the file: {error:?}.")
    };

    // But the Result<T, E> enum has many helper methods in order to avoid 
    // those match nestings
    // unwrap is one of them
    // let greeting_file_2 = File::open("test.txt").unwrap();
    // if the file does not exit the unwrap method panics! for us, uncomment and see
    let greeting_file_3 = File::open("src/main.rs").expect("test.txt should exist.");
    // also panics but with our cusotm error, because in this case
    // the operation should always succeed.


    // let's use the small function possible for the example
    // of reading a username from file (check below)
    let username = read_username_from_file_3(String::from("src/main.rs"));
    match username {
        Ok(username) => println!("username founded... {username}"),
        Err(error) => panic!("There was an awful error {error}")
    }

    // the fs:: option
    let username_simple = read_username_from_file_4(String::from("src/main.rss")).expect("A src/main.rss was expected");

    ()

}

// In summary, things that return a Result might fail
// so all cases should be handled
// a common pattern to do so is ERROR PROPAGATION
// meaning, sending to the called code (the code that called read_username_from_file)
// the results (Ok() or Error()) for it to handle
// Let's consider the case
fn read_username_from_file() -> Result<String, io::Error>{
    let username_file_result = File::open("username.txt");
    let mut username_file = match username_file_result{
        Ok(file) => file,
        Err(error) => return Err(error)
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username){
        Ok(_) => Ok(username),
        Err(error) => Err(error)
    }
}

// this patter of propagation is so common in Ruest 
// that we have the question mark ? operator to handle them.

// let's implement a funtion read_username_from_file but
// with the ? operator
fn read_username_from_file_2() -> Result<String, io::Error>{
    let mut username_from_file = File::open("username.txt")?;
    // the ? means that if the result is Ok() then Ok(file) will be returned
    // and if it fails then Err(error) will be returned **from the whole function** (like ending with return X)
    // but in this case the error type will the then one of the return type of the
    // current function.

    let mut username = String::new();
    username_from_file.read_to_string(&mut username)?;
    Ok(username)
    // the question mark operators ? makes our functions simpler to write
    // eliminating boilerplate code

}

// But it can be even simpler, as we can "concatenate" or chain some methods
fn read_username_from_file_3(filepath: String) -> Result<String, io::Error>{
    let mut username = String::new();
    let mut username_from_file = File::open(filepath)?.read_to_string(&mut username)?;
    Ok(username)
}

// And finally there is even a shorter way, provided by the Standard Library
// this function opens the file, creates a new String, reads the content
// put the content into a string and returns it.
use std::fs;
fn read_username_from_file_4(filepath: String)-> Result<String, io::Error>{
    fs::read_to_string(filepath)
}

// We can also use the question mark operator in an Option<T> value
// And the same as with Result, WE CAN ONLY USE ? ON OPTION, IN A FUNCTION
// THAT ALSO RETURNS AN OPTION (That is why at the end of main() i could not return ?)
// since main() has a return type of ()


// When failure is expected, it’s more appropriate to return a Result than to make a panic! call.