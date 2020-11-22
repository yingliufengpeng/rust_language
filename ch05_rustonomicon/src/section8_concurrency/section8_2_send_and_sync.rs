//!
//! # Send and Sync
//!
//!     A type is Send if it is safe to send it to another thread.
//!     A type is Sync if it is safe to share between threads (&T is Send).
//!


struct MyBox(*mut u8);

unsafe impl Send for MyBox {}
unsafe impl Sync for MyBox {}


// I have some magic semantics for some synchronization primitive!
struct SpecialThreadToken(u8);

impl !Send for SpecialThreadToken {}
impl !Sync for SpecialThreadToken {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_001() {

    }


}