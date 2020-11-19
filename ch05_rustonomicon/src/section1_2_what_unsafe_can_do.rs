

//!
//! What Unsafe Rust Can Do
//!
//!     # Dereference raw pointers
//!     # Call unsafe functions (including C functions, compiler intrinsics, and the raw allocator)
//!     # Implement unsafe traits
//!     # Mutate statics
//!     # Access fields of unions
//!
//! Rust is otherwise quite permissive with respect to other dubious operations. Rust
//!     considers it "safe" to
//!
//! # Deadlock
//! # Have a race condition
//! # Leak memory
//! # Fail to call destructors
//! # Overflow integers
//! # Abort the program
//! # Delete the production database
//!

