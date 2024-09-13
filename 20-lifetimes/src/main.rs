// Lifetimes are a type of generic
// Lifetime annotations don’t change how long any of the references live. 
// They describe the relationships of the lifetimes of multiple references to each other

// The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. 
// the string slice returned from the function will live at least as long as lifetime 'a

// we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints.

// When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y. In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y. 

// Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. 

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}


fn main() {
    let string1 = String::from("abcd");
    let string2 : &str = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);



}