
//!
//! The Borrow trait is used when you're writing a data structure, and you want to use either an owned
//! or borrowed type as synonymous for some purpose.
//!
//! The AsRef trait is a conversion trait. It's used for converting some value to a reference in generic
//! code
//!
//! Choose Borrow when you want to abstract over different kinds of borrowing, or when you're building a
//! data structure that treats owned and borrowed values in equivalent ways, such as hashing and comparison.
//!
//! Choose AsRef when you want to convert something to a reference directly, and you're writing generic code

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::borrow::Borrow;
    use std::fmt::Display;

    fn foo<T: Borrow<i32> + Display>(a: T) {
        println!("a is borrowed {}", a);
    }

    fn foo2<T: AsRef<str>>(s: T) {
        let slice = s.as_ref();
        println!("{}", slice);
    }

    #[test]
    fn test_002() {
        let s = "Hello".to_string();
        foo2(s);
    }

    #[test]
    fn test_001() {

        let mut map = HashMap::new();
        map.insert("Foo".to_string(), 42);

        assert_eq!(map.get("Foo"), Some(&42));

        let i = 5;
        foo(&i);
        foo(i);


    }

}