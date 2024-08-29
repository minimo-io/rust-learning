fn main() {
    // a loop
    let s = loop{
        println!("Do this forever and ever...");
        break "ssssd"
    };
    println!("String returned: {s}");
    let mut remaining = 10;
    'this_is_a_loop_label: loop{
        
        println!("Remaining is: {remaining}");
        remaining -= 1;
        if remaining == 5{
            break 'this_is_a_loop_label;
        }
        
    }
    // a while
    let mut number = 5;
    while number > 0{
        println!("{number}");
        number -= 1;
    }

    // and a for
    let elements:[u32;3] = [1, 24, 515];
    for element in elements{
        println!("this value is:  {element}");
    }
    // a for loop using a range
    for number in (1..4).rev(){
        println!("{number}");
    }
}
