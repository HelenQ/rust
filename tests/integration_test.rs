use rust::add_two;

#[test]
fn it_add_two() {
    assert_eq!(4, add_two(2, 2));
}
// cargo test --test integration_test