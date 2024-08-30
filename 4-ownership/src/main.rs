// So finall, some ownership concepts.
// In rust memory is managed by a set of ownership rules, not a garbage collector.
// The stack and the heap are parts of memory available at runtime.
// The Stack: Stores values in the order it gets them and removes them in the opposite order.
// last-in, first-out.
// All data on the stack must have a fixed size.
// THe Heap: is for data unknown at compile time, or data which size need to change.

// Pushing data to the stack is faster, because the allocator does not need to search for a place.
// Accesing data on the heap requires following a pointer.
// The purpose of ownership is managing data on the heap.

// Rules:
// - Each value has an owner
// - There can only be one owner at a time
// - When the owner goes out of scope, the value will be dropped

fn main() {
    let mut s = String::from("The Magnificent Seven");
    s.push_str(", The Clash");
    println!("{s}");
}
