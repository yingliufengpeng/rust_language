
//!
//! # Coercions
//!


#[cfg(test)]
mod tests {
    trait Trait {}

    fn foo<X: Trait>(t: X) {}

    impl<'a> Trait for &'a i32 {}

    impl <'a> Trait for &'a mut i32 {}

    #[test]
    fn test_001() {
        let t = &0;
        foo(t);

        let t: &mut i32 = &mut 0;
        foo(t);
    }

}