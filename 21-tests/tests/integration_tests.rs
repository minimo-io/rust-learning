use adder::add_three;

mod common;

#[test]
fn it_adds_to_baby() {
    common::setup();
    let result = add_three(27);
    assert_eq!(result, 30);
}