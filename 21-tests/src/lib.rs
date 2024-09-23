// Tests fail when something in the test function panics
// Tests run in parallel threads, to configure this we must send 
// an option to the binary, like this: cargo test -- --test-threads=1

// Each physical CPU core can typically handle multiple threads simultaneously
// so the maximum number of threads you can create is likely to be higher than the number of cores.

// In mac we can check this like so: sysctl -n hw.ncpu

// We can also specify which which test to run passing an argument to the cargo test like this:
// cargo test [pattern_with_test_or_test_module_name]

// We can #[ignore] a time consuming test
// if we insist on running the ignored test we can do this:
// cargo test -- --ignored

// To run all tests and don't give a f about ignored tests run:
// cargo test -- --include-ignored

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(n: usize)-> usize{
    n + 2
}
pub fn add_three(n:i32)->i32{
    n + 3
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

#[cfg(test)]
mod some_other_stuff_to_try{
    use crate::add_three;

    #[test]
    #[ignore]
    fn it_adds_three(){
        let result_of_sum_3 = add_three(10);
        assert_eq!(result_of_sum_3, 23);
    }
}
