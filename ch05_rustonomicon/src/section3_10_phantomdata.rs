
//!
//! # PhantomData
//!     We do this using PhantomData, which is special marker type. PhantomData consumes no space,
//!         but simulates a field of the given type fro the purpose of static analysis.
//!
//!
//!


#[cfg(test)]
mod tests {

    use super::*;
    use std::marker;

    struct Iter<'a, T: 'a> {
        ptr: *const T,
        end: *const T,
        _marker: marker::PhantomData<&'a T>
    }

    /// In order to tell dropck that we do own values of type T, and therefore may drop some T's when we
    ///     drop, we must add an extra PhantomData saying exactly.
    #[derive(Debug)]
    struct MyVec<T> {
        data: *const T, // *const for variance!
        len: usize,
        cap: usize,
        _marker: marker::PhantomData<T>
    }

    #[test]
    fn test_001() {
        println!("OK");
        let m = 3;
        let r = MyVec {
            data: &m as *const i32,
            len: 1,
            cap: 10,
            _marker: marker::PhantomData::default(),
        };
        println!("{:?}", r);


        println!("{}", m);

    }


    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
