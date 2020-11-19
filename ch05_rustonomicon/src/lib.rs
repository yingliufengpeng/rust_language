//!
//! The Dark Arts of Unsafe Rust.
//!

// #![forbid(unsafe_code)]


pub mod section0_0_introduction;
pub mod section1_1_how_safe_and_unsafe_interact;
pub mod section1_2_what_unsafe_can_do;


#[cfg(test)]
mod tests {

    #[test]
    fn test_001() {
        unsafe {
            println!("KK");
        };
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
