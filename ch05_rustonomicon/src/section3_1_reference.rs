
//!
//!
//! # References
//!    Shared reference: &
//!    Mutable reference: &mut
//!
//! # rules
//!    A reference cannot outlive its referent
//!    A mutable reference cannot be aliased


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut v = vec![1, 2, 3];

        // let x = &v[0];
        v.push(3);
        println!("{:?}", v);
    }
}
