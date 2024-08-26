// Constants can go in the global scope
const THREE_HOURS_IN_SECOINDS: i32 = 60* 60* 3;

fn main() {
    
    // Inmmutable variables can be shadowed, changing type (good to reuse)
    // Mutable variables "mut" cannot change type
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope: {x}");
    }
    println!("The value of x is: {x}");
   

    println!("{}", THREE_HOURS_IN_SECOINDS)
}