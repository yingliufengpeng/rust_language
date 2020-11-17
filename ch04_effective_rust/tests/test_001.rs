//! Cargo will ignore files in subdirectories of the tests/ directory. Therefore shared
//! modules in integrations tests are possible. For example tests/common/mod.rs is not
//! separately compiled by cargo can be imported in every test with mod common;
//!
//! Note, when building integration tests, cargo will not pass test attribute to the compiler.
//! It means that all parts in cfg(test) won't be include in the build used in your
//! integration tests.


extern crate ch04_effective_rust as me;



#[test]
fn test_001() {
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    me::show();
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    println!("ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");

}