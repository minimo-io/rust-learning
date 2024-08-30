// a general template, for an object like structure
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_account: u64
}
fn main() {

    // the instance does not need to follow the order
    let mut user_pepe = User{
        active: true,
        username: String::from("Pepe"),
        email: String::from("pepe@pepito.com"),
        sign_in_account: 214,
    };
    println!("{}", user_pepe.email);

    // if the instance is mut we can change its values
    // the whole instance must be mutable, only fields is not available.
    user_pepe.active = false;
    
    // -----
    let jasmin_email = String::from("jasmin@pepe.com");
    let jasmin_name = String::from("Jasmin");

    let user_jasmin = build_user(jasmin_email, jasmin_name);
    println!("{}", user_jasmin.email);   

    // -----
    // Shorthand for copying fields from another instance, changing others
    let user_nico = User{
        email: String::from("nicolas@minimo.io"),
        ..user_pepe
    };
    // the ..user_pepe notation should be last, and will populate all fields not explicitly specified
    println!("{}", user_nico.email);
    println!("Sign in account: {}", user_nico.sign_in_account);
    // Beware that user_pepe will no longer be availble, since some of its fileds are owned by user_nico
    // If we had used only types with the Copy trait (integers, chars) from the user_pepe
    // the it would be still available.

    // -----
    // Each struct or tuple struct we define IS its own type
    // Here is a Tuple Struct
    struct Color( i32, i32, i32 );
    let black = Color(0, 0, 0);

    // -----
    // UNIT LIKE STRUCTS are also a thing
    struct SomeType;
    let something= SomeType;
}

fn build_user(email: String, username: String) -> User{
    User{
        active: true,
        username,
        email,
        sign_in_account: 1
    }
}
