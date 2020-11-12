macro_rules! X { () => {}; }


#[cfg(test)]
mod tests {

    mod a {
        X!();
    }

    mod b {

        macro_rules! X2 { () => { println!("Kk"); }; }
        fn show() {

            X2!();
        }
    }

    mod c {
        X!();
    }


    mod d {

        mod a {

        }
        macro_rules! X { () => { Y!(); }; }
        mod b {
            // X!() // defined, but Y! is undefined
        }

        macro_rules! Y { () => {}; }
        mod c {
            X!(); // defined, and so is Y!
        }
    }

    mod e {

        mod a {
            // X!(); // undefined
        }

        #[macro_use]
        mod b {
            macro_rules! X { () => {}; }
        }

        mod c {


            X!(); // defined
        }
    }

    #[test]
    fn test_001() {


    }
}