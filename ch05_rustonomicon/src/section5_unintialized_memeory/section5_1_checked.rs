
//!
//! # Checked Uninitialized Memory
//!
//!
//!



#[cfg(test)]
mod tests {

    #[test]
    fn test_005() {
        let x: i32;

        // Rust doesn't understand that this branch will be taken unconditionally,
        // because it relies on actual values.
        loop {
            // But it doesnt' understand that it will only be taken once because
            // we unconditionally break out of it. Therefore `x` doesn't need to
            // be marked as mutable

            x = 0;
            break;
        }

        println!("{}", x);
    }

    #[test]
    fn test_004() {
        let x: i32;
        if true {
            x = 1;
        }

        // println!("{}", x);
    }

    #[test]
    fn test_003() {
        let x: i32;
        if true{
            x = 1;
        } else {
            x = 2;
        }

        println!("{}", x);
    }

    #[test]
    fn test_002() {
        let x: i32;
        if true {
            x = 1;
            println!("{}", x);
        }
        // Don't care that there are branches where it's not initialized
        // since we don't use the value in those branches
    }

    #[test]
    fn test_001() {
        let x: i32;

        if true {
            x = 1;
        } else {
            x = 2;
        }

        println!("{}", x);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}