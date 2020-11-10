#[cfg(test)]
mod tests {

    use std::ops::Deref;
    use std::rc::Rc;

    #[derive(Debug)]
    struct DerefExample<T> {
        value: T,
    }

    impl <T> Deref for DerefExample<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    struct Foo;

    impl Foo {
        fn foo(&self) {
            println!("FOO");
        }
    }

    #[test]
    fn test() {

        let x = DerefExample { value: 3 };
        println!("{:?}", x);

        let y = *x;

        println!("{}", y);

        fn foo(s: &str) {
            // borrow a string for a second
        }

        // String implements Deref<Target=str> .
        let owned = "Hello".to_string();

        // Therefore, this works
        foo(&owned);

        // That's it. This rule is one of only places in which Rust does
        // an automatic conversion for you, but it adds a lot of flexibility.


        let counted = Rc::new(owned);
        foo(&counted);


        fn foo2(s: &[i32]) {
            // Borrow a slice for second
        }

        // Vec<T> implements Deref<Target=[T]>.
        let owned = vec![1,2, 3];
        foo2(&owned);

        let v = [1, 2, 3];
        foo2(&v);


        let f = Foo;

        f.foo();
        (&f).foo();
        (&&f).foo();
        (&&&&&&&&&&&&&f).foo();
    }
}
