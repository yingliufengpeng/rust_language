use crate::phrases as sayings;
// use phrases::english::farewells;
// use super::super::foo;
// use phrases::farewells;
// use crate::foo;

use sayings::japanese::greetings as ja_greetings;
use sayings::japanese::farewells::*;
use sayings::english::{self as me , greetings as en_greetings, farewells as en_farewells};

#[test]
fn test_001() {

    // println!("Hello in English: {}", phrases::english::greetings::hello());
    // println!("Goodbye in English: {}", farewells::goodbye());
    //
    // println!("Hello in Japanese: {}", phrases::japanese::greetings::hello());
    // println!("Goodbye in Japanese: {}",farewells::goodbye());
    //
    // foo();

    println!("Hello in English; {}", en_greetings::hello());
    println!("And in Japanese: {}", ja_greetings::hello());
    println!("Goodbye in English: {}", me::farewells::goodbye());
    println!("Again: {}", en_farewells::goodbye());
    println!("And in Japanese: {}", goodbye());
    // self::english::farewells::goodbye();


}