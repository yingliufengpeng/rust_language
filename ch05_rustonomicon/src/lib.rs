#![feature(label_break_value)]
//!
//! # The Dark Arts of Unsafe Rust.
//!
//!     # 1 Meet Safe and Unsafe
//!         ## 1.1 How Safe and Unsafe Interact
//!         ## 1.2 What Unsafe Can Do
//!         ## 1.3 Working with Unsafe
//!
//!     # 2 Data Layout
//!         ## 2.1 repr(Rust)
//!         ## 2.2 Exotically Sized Types
//!         ## 2.3 Other reprs
//!

// #![forbid(unsafe_code)]


pub mod section0_0_introduction;
pub mod section1_1_how_safe_and_unsafe_interact;
pub mod section1_2_what_unsafe_can_do;
pub mod section1_3_working_with_unsafe;
pub mod section2_1_data_layout;
pub mod section2_2_exotically_sized_types;
pub mod section2_3_other_reprs;
pub mod section3_1_reference;
pub mod section3_2_aliasing;
pub mod section3_3_lifetimes;
pub mod section3_4_limits_of_lifetimes;
pub mod section3_5_lifetime_elision;
pub mod section3_6_unbounded_lifetimes;


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
