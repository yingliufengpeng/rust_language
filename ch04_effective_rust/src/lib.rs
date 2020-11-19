//! The `ch04_effective_rust` crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, ch04_effective_rust::add_two(2));
//! ```

/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use ch04_effective_rust::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```

#[macro_use]
extern crate hello_world_drive;

mod section01_the_stack_and_heap;
mod section02_testing;
mod section02_testing2;
mod section03_conditional_compilation;
mod section04_documentation;
mod section05_iterators;
pub mod section06_concurrency;
pub mod section08_choosing_your_guarantees;
mod section09_ffi;
mod section10_borrow_and_as_ref;
mod section_11_release_channels;
mod section_12_using_rust_without_the_standard_library;
mod section_13_procedural_macros_and_custom_derive;

pub fn show() {
    println!("ok");
}



pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }



}