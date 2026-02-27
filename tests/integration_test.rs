use rust_example::{checked_add, welcome};

#[test]
fn checked_add_integration() {
    assert_eq!(checked_add(10, 15).expect("no overflow"), 25);
}

#[test]
fn welcome_integration() {
    let msg = welcome("Alice").expect("valid name should pass");
    assert!(msg.contains("Alice"));
}
