
//!
//! # Drop Check
//!
//!  The reason is that the borrow checking analysis of main doesn't know about the internals of each
//!  Inspector's Drop implementation. As far as the borrow checker knows while it is analyzing main,
//!  the body of an inspector's destructor might access that borrowed data.
//!
//!  For a generic type to soundly implement drop, its generics arguments must strictly outlive it.
//!
//!
//!  An Escape Hatch
//!
//!
//!  It is sometimes obvious that no such access can occur, like the case above. However, when dealing
//!     with a generic type parameter, such access can occur indirectly. Examples of such indirect access
//!     are:
//!         invoking a callback
//!         via a trait method call.
//!


#[cfg(test)]
mod tests {
    use std::fmt::Display;
    use std::fmt;

    #[derive(Debug)]
    struct Inspector<'a>(&'a u8);

    impl <'a> Drop for Inspector<'a> {
        fn drop(&mut self) {
            println!("I was only {} days from retirement!", self.0);
        }
    }

    #[derive(Debug)]
    struct World<'a> {
        inspector: Option<Inspector<'a>>,
        days: Box<u8>,
    }

    #[derive(Debug)]
    struct Inspector2<'a>(&'a u8, &'static str);

    unsafe impl<#[may_dangle] 'a> Drop for Inspector2<'a> {
        fn drop(&mut self) {
            println!("Inspector(_, {}) knows when *not* to inspect.", self.1);
        }
    }

    #[derive(Debug)]
    struct World2<'a> {
        days: Box<u8>,
        inspector: Option<Inspector2<'a>>,
    }

    #[derive(Debug)]
    struct Inspector3<'a, 'b, T, U: Display>(&'a u8, &'b u8, T, U);

    unsafe impl<#[may_dangle]'a, #[may_dangle] 'b, T, U: Display> Drop for Inspector3<'a, 'b, T, U> {
        fn drop(&mut self) {
            println!("Inspector({}, _, _, {})", self.0, self.3);
        }
    }

    #[derive(Debug)]
    struct World3<'a > {
        days: Box<u8>,
        inspector: Option<Inspector3<'a, 'a, u8, u8>>,
    }

    struct Inspector4<T>(T, &'static str, Box<for <'r> fn(&'r T) -> String>);

    impl<T> Drop for Inspector4<T> {
        fn drop(&mut self) {
            // The `self.2` call could access a borrow e.g. if `T` is `&'a _`.
            println!("Inspector({}, {}) unwittingly inspects expired data.",
                     (self.2)(&self.0), self.1);
        }
    }

    struct Inspector5<T: fmt::Display>(T, &'static str);

    impl<T: fmt::Display> Drop for Inspector5<T> {
        fn drop(&mut self) {
            // There is a hidden call to `<T as Display>::fmt` below, which
            // could access a borrow e.g. if `T` is `&'a _`
            println!("Inspector({}, {}) unwittingly inspects expired data.",
                     self.0, self.1);
        }
    }

    #[test]
    fn test_004() {
        let mut world = World3 {
            inspector: None,
            days: Box::new(3),
        };


        // println!("{:?}", world);

        world.inspector = Some(Inspector3(&world.days, &23, 3, 3));
    }

    #[test]
    fn test_003() {
        let mut world = World2 {
            inspector: None,
            days: Box::new(3),
        };


        println!("{:?}", world);

        world.inspector = Some(Inspector2(&world.days, "Ok"));
    }


    #[test]
    fn test_002() {

        let mut world = World {
            inspector: None,
            days: Box::new(3),
        };


        println!("{:?}", world);

        // world.inspector = Some(Inspector(&world.days));
        // Let's say `days` happens to get dropped first.
        // Then when Inspector is dropped, it will try to read free'd memory!

        // note: values in a scope are dropped in the opposite order they are created


    }

    #[test]
    fn test_001() {
        let x: i32;
        let y: i32;
        {
            let x: u32;
            {
                let y: u32;
            }
        }


    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

