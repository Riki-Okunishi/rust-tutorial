
mod chapter11_1;

#[test]
fn it_passed() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn it_failed() {
    assert_eq!(2 + 2, 5)
}

#[test]
fn it_raise_panic() {
    panic!("Make this test fail");
}

#[test]
fn test_chapter11() {
    chapter11_1::chapter11_1();
}