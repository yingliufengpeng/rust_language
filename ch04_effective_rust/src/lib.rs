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


mod section01_the_stack_and_heap;
mod section02_testing;
mod section03_conditional_compilation;
mod section05_iterators;
mod section10_borrow_and_as_ref;

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