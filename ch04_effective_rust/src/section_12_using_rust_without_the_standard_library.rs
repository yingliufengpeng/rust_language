//!
//! Rust’s standard library provides a lot of useful functionality,
//! but assumes support for various features of its host system:
//! threads, networking, heap allocation, and others. There are
//! systems that do not have these features, however, and Rust
//! can work with those too! To do so, we tell Rust that we don’t
//! want to use the standard library via an attribute: #![no_std].
//!
//!
//! Much of the functionality that's exposed in the standard library
//! is also available via the core crate.
//! When we're using the standard library, Rust automatically brings std
//! into scope, allowing you to use its features without an explicit import
//!. By the same token, when using #![no_std], Rust will bring core into scope
//! for you, as well as it prelude. This means that lot of code will just work.


#[cfg(test)]
mod tests {

    #[test]
    fn test_001() {

    }
}