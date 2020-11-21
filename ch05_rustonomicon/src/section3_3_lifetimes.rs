//!
//!
//! # Lifetimes
//!
//!     One particularly interesting piece of sugar is that each let statement
//!         implicitly introduces a scope.
//!
//! # The area covered by a lifetime
//!
//!
//!

#[derive(Debug)]
struct X<'a>(&'a i32);

impl Drop for X<'_> {
    fn drop(&mut self) {

    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_004() {
        let mut data = vec![1, 2, 3];
        // This mut allows us to change where the reference points to '
        let mut x = &data[0];
        println!("{}", x);  // Last use of this borrow

        data.push(4);
        x = &data[2]; // We start a new borrow here
        println!("{}", x);

    }

    #[test]
    fn test_003() {
        let mut data = vec![1, 2, 3];
        let x = &data[0];

        let r = 111 % 2 == 0;

        if r {
            println!("{}", x);
            data.push(4);
        } else {
            // There's no use of `x` in here, so effectively the last  use if the
            // creation of x at the top of the example.
            data.push(34);
        }

    }

    #[test]
    fn test_002() {

        let mut data = vec![1, 2, 3];
        let x = X(& data[0]);

        println!("{:?}", x);

        // data.push(4);


    }

    fn as_str<'a>(data: &'a u32) -> String {
        let s = format!("{}", data);
        s
    }

    #[test]
    fn it_works() {

        let x = 0;
        let y = &x;
        let z = &y;
        println!("{}", z);

        'a: {
            let x = 3;
        }

        'b: {
            let y = 210;
            println!("{}", y);
        }
    }
}
