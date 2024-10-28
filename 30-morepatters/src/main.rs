fn main() {
    let some_var = 30;
    // Here we have a multiple pattern
    // for literals
    match some_var {
        1 | 30 => println!("x is 1 or 30: {}", some_var),
        20 => println!("x is {some_var}"),
        _ => println!("None"),
    }

    // matching ranges in rust
    // in this case an inclusive range 1..=30
    // Remember my boy than in Rust ranges like 1..30 will match any value from
    // 1 to 29. If we want to include 30 we will do the following range:
    // This ranges matches are only compiler ok for numerics and chars.
    match some_var {
        1..=30 => println!("Matched the inclusive range 1..=30"),
        _ => println!("Someting else, unknown... (mystery)"),
    }

    // An example of inclusive ranges for the char value
    let some_char = '\u{00E0}';
    match some_char {
        'a'..='j' => println!("Early ASCII letter"),
        'k'..='z' => println!("Late ASCII letter"),
        _ => println!("Some other char: {some_char}"),
    }

    // Pattern deconstructing for structs
    let p = Point { x: 19, y: 22 };
    let Point { x: a, y: b } = p;
    // this would be a shorthand for matching structs
    let Point { x, y } = p;
    println!("{x}"); // print the x just deconstructed
    dbg!(p);

    // And now a match for an struct
    let p1 = Point { x: 0, y: 22 };

    match p1 {
        Point { x, y: 0 } => println!("Point on the x axis: {}", x),
        Point { x: 0, y } => println!("Point in the y axis: {}", y),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }

    // Now a match with enums
    // Deconstructing examples for some enum variants
    // For nested deconstructing examples check:
    // https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-nested-structs-and-enums
    let m = Message::ChangeColor(32, 23, 100);
    match m {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}")
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}")
        }
    }

    // There are also complex deconstructs with tuples and Structs
    // so we can use their values separately
    // Check it out
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 10, y: -10 });

    // Now let's ignore parts of patters
    // For example the underscore, used as a catch all in a match expression
    // can be used to ignore values in ANY patter
    // for example in this function signature:
    fn foo(_: i32, y: i32) {
        println!("I insist in only using the y parameter {y}");
    }
    foo(3, 5);

    // We can also ignore parts of a value with for example Some(_)
    // More here: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#ignoring-parts-of-a-value-with-a-nested-_

    // Also we can ignore multiple values within a patter
    let some_tuple = (2, 41, 515, 132, 51);
    match some_tuple {
        (first, _, third, _, fifth) => println!("Some numbers: {first}, {third}, {fifth}"),
    }

    // We can create vars with an _ to avoid a warning from the compiler for not using it
    let _shh = 5; // no warning, but we still bind a value to the _ssh var
    let shh = 131; // warning
    let some_shh = Some(shh);
    // Also, as _ does not bind (assign),
    // so we can still print some_ssh after the if let
    // as no ownership is transferred
    if let Some(_) = some_shh {
        println!("found a number");
    }

    println!("A number {some_shh:?}");
    // It would be different if we do `if let Some(_s) = some_shh { }` <<< ownership transferred

    // Now, more ingnoring options
    // For example, we can ignore the rest of a pattern with ..
    // in order to avoid being specific about each _
    struct AnotherPoint {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = AnotherPoint { x: 0, y: 0, z: 0 };
    match origin {
        AnotherPoint { x, .. } => println!("x is {x}"),
    }

    // Also interesting ignoring parts of tuples for example
    let tiny_tuple = (231, 415, 241, 5, 51, 0);
    match tiny_tuple {
        (first, .., last) => {
            println!("First ({first}) and last ({last}) value of a tuple, ignoring the rest.")
        }
    }

    // Match Guards, an additional if condition
    // specified after the match arm
    // this condition can use variables created in the pattern
    // Let's check it out
    let num = Some(4);
    match num {
        Some(x) if x % 4 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
    // Downside is that Rust does not check for exhaustiveness when a
    // match guard is involved

    // We can also specify multiple patterns in a Match Guard
    let xx = 4;
    let yy = false;
    match xx {
        4 | 5 | 6 if yy => println!("Yes!"),
        _ => println!("Nop :/"),
    }
    // The first arm of the above code matches if xx is 4, 5 or 6 and yy=true
    // and not only if 6 and yy=true

    // Finally, there is the @ operator
    // for testing a value AND saving it in a variable within one pattern.
    // More here: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#-bindings
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
