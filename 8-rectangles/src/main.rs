struct Rectangle{
    width: u32,
    height:u32
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    ); // we want to borrow (pass a reference) instead of ownership to be able to keep using rect1

}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.height * rectangle.width
}