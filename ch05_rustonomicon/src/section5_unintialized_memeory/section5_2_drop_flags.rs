
//!
//! # Drop Flags
//!


#[cfg(test)]
mod tests {

    #[test]
    fn test_002() {
        let mut x = Box::new(4);  // x was uninit; just overwrite.
        if true {
            drop(x);        // x gets moved out; make x uninit.
        } else {
            println!("{}", x);
            drop(x);    // x gets moved out; make x uninit;
        }

        x = Box::new(4);  // x was uninit; just overwrite.

        println!("{}", x);   // x goes out of scope; x was init; Drop x
    }

    #[test]
    fn test_001() {

        let mut x = Box::new(3); // x was uninit; just overwrite
        let mut y = x;  // y was uninit; just overwrite and make x uninit;

        x = Box::new(3); // x was uninit; just overwrite.

        y = x;      // yas init; Drop y, overwrite it, and make x uninit!

                    // y goes out of scope; y wa init; Drop it
                    // x goes out of scope; x wa uninit; do nothing



    }
}