
//!
//! The standard library has a number of unsafe functions, including:
//! slice::get_unchecked, which performs unchecked indexing, allowing
//! memory safety to be freely violated.
//! mem::transmute reinterprets some value as having a given type, bypassing type safety in
//! arbitrary ways (see conversions for details).
//! Every raw pointer to a sized type has an offset method that invokes Undefined Behavior if
//! the passed offset is not "in bounds".
//! All FFI (Foreign Function Interface) functions are unsafe to call because the other language
//! can do arbitrary operations that the Rust compiler can't check.
//!
//! As of Rust 1.29.2 the standard library defines the following unsafe traits (there are others,
//! but they are not stabilized yet and some of them may never be):
//!
//!      Send is a marker trait (a trait with no API) that promises implementors are safe to send (move) to another thread.
//!
//!      Sync is a marker trait that promises threads can safely share implementors through a shared reference.
//!
//!      GlobalAlloc allows customizing the memory allocator of the whole program.
//!
//!
//!
//!








#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
