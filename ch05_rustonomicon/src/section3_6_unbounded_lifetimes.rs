
//!
//! # Unbounded Lifetimes
//!
//!
//! # Given a function, any output lifetimes that don't derive from inputs are unbounded.
//!

fn get_str<'a>() -> &'a str {
    "aa"
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
