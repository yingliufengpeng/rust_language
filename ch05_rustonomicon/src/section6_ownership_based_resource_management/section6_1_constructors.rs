
//!
//! # Constructors
//!
//!
//!
//!


struct Foo {
    a: u8,
    b: u32,
    c: bool,
}

enum Bar {
    X(u32),
    Y(bool),
}

struct Unit;

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_001() {
        let foo = Foo { a: 0, b: 1, c: false };
        let bar = Bar::X(0);
        let empty = Unit;


    }

}
