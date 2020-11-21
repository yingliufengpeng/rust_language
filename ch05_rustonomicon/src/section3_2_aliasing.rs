
//! # Aliasing
//!
//! # Why Aliasing Matters
//!     # keeping values in registers by proving no pointers access the values's memory
//!     # eliminating reds by proving some memory hasn't been written to since last we read it
//!     # eliminating writes by some memory is never read before the next write to it
//!     # moving or reordering reads and writes by proving they don't depend on each other
//!
//!
//!
//!
//!



#[cfg(test)]
mod tests {

    fn compute3(input: &u32, output: &mut u32) {
        let mut temp = *output;
        if *input > 10 {
            temp = 1;
        }

        if *input > 5 {
            temp = 2;
        }

        *output = temp;
    }

    fn compute2(input: &u32, output: &mut u32) {
        if *input > 10 {
            *output > 1;
        }

        if *input > 5 {
            *output *= 2;
        }
    }

    fn compute(input: &u32, output: &mut u32) {
        let cached_input = *input;
        if cached_input > 10 {
            *output = 1;
        }

        if cached_input > 5 {
            *output = 2;
        }
    }

    #[test]
    fn test_001() {


    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
