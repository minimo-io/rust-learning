#[derive(Debug)]
struct Rectangle{
    width: u32,
    height:u32
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle{
        width: dbg!(30 * scale),
        height: 50
    };
    println!("rect 1 is {rect1:#?}"); // :? tells println! that we want to use an output called Debug
    // dbg macro outputs to the stderr instead of the stadout
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    ); // we want to borrow (pass a reference) instead of ownership to be able to keep using rect1

}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.height * rectangle.width
}