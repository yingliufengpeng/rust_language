
//!
//!  # Unwinding
//!
//!  # Rust has a tiered error-handling scheme:
//!
//!     If something might reasonably be absent, Option is used.
//!     If something goes wrong and can reasonably be handled, Result is used.
//!     If something goes wrong and cannot reasonably be handled, the thread panics.
//!     If something catastrophic happens, the program aborts.
//!

pub mod section7_1_exception_safety;
pub mod section7_2_poisoning;

#[cfg(test)]
mod tests {

    #[test]
    fn test_001() {


    }

}
