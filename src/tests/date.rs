use crate::add;

// BASIC TESTS
// @TODO: Update to test date endpoint responses
#[test]
fn add_test() {
    let result = add(2, 2);
    assert_eq!(result, 4);

    let result = add(3, 2);
    assert_ne!(result, 4);
}
