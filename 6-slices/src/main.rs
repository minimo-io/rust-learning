fn main() {
    // a slice is a reference to a part of a string
    let s = String::from("Hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("/{hello}/");
    println!("/{world}/");
    // The type that signifies “string slice” is written as &str
    // starting_index is the first position in the slice and ending_index is one more than the last position in the slice
    // we can take slices from literals and string values
    // There are other slices, like array slices, that work the same
    let a = [1,2,3,4,61];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
