// methods are defined within the context of a struct 
// (also enums and traint objects) and they work like functions
// their first parameter is always self
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height:u32
}
impl Rectangle {
    // All functions here are called associated functions
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
    // Associated functions without &self are not methods.
    // Like String::from, they create a new instance of the struct
    // The Self in the return type are aliases of what is after the struct definition
    // To call these type of associated functions we use ::
    fn square(size:u32) -> Self{
        Self { width: size, height: size }
    }
    // methods can also be in different impl blocks
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50
    };
    let rect2 = Rectangle{width: 10, height:49};

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let some_square = Rectangle::square(40);
    println!("A squere of {}", some_square.area());
}
