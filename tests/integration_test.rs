use playground;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, playground::maths::add_two(2));
}
