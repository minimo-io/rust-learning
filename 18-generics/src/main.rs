// Generics allow us to replace specific types with a placeholder that represents multiple types to remove code duplication.

// First we are going to create an abstract function
// to get the largest number on a vector
fn largest_number(list: &[i32]) -> &i32{
    let mut largest = &list[0];
    for item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}
// when using generics in functions, we do it in the function's signature
// It is a convention to name Rust generic parameters with a single letter, like T, but we could use other.
// any UpperCamelCase would work but stick to conventions.

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];
    for item in list{
        if item > largest{
            largest = item
        }
    }
    largest
}

// Here is how we read the above function:
// The function largest is generic over some type T. This function has one parameter named list, which is a slice of values of type T. The largest function will return a reference to a value of the same type T. https://doc.rust-lang.org/book/ch10-01-syntax.html

// But without a Trait that LIMITS WHICH TYPES T CAN BE
// the function will return: 
// error[E0369]: binary operation `>` cannot be applied to type `&T`
// help: consider restricting type parameter `T`
// By following the help text’s suggestion, we restrict the types valid for T to only those that implement PartialOrd

// So we use the trait : std::cmp::PartialOrd for T.

// we can also create structs with generic types
#[derive(Debug)]
struct SomePoint<T> {
    x: T,
    y: T,
}



fn main() {
    let all_numbers = vec![34, 50, 25, 100, 65];
    let result = largest_number(&all_numbers);
    println!("{result}");

    // with our generic function with generics
    let all_numbers_2 = vec![13, 5151, 4561, 2];
    let result2 = largest(&all_numbers_2);
    println!("{:#?}", result2);

    // our struct
    let some_point_1 = SomePoint{x:1, y:515};
    dbg!("{}", some_point_1);
    let some_point_2 = SomePoint{x:212.51, y:1.1111112};
    dbg!("{}", some_point_2);
    // we could also define generic structures
    // with more than one type
    struct Point<T, U> {
        x: T, // could be an i32
        y: U, // could be a f64
    }

    // also generics are present in the emums who hold data types
    enum Option<T> {
        Some(T),
        None,
    }    
    
}
