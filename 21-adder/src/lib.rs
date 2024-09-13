// Tests fail when something in the test function panics

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(n: usize)-> usize{
    n + 2
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    // use core::panic;

    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another(){
        // panic!("You will fail!");
    }

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle{
            width: 8,
            height: 10
        };

        let smaller: Rectangle = Rectangle{
            width: 5,
            height: 1
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_adds_two(){
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    // we can use tests that return a Result
    #[test]
    fn it_works() -> Result<(), String>{
        let result = add(2, 2);
        if result == 4{
            Ok(())
        }else{
            Err(String::from("two plus two does not = 4"))
        }
    }
}
