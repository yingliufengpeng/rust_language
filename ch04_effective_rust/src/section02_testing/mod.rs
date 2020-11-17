


mod tests;

#[test]
#[should_panic(expected="false")]
fn test_001() {

    assert!(false);
}

#[test]
#[ignore]
fn expensive_test() {

}


#[test]
#[ignore]
fn expensive_test2() {

}