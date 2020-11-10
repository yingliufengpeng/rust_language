#[cfg(test)]
mod tests {
    trait Foo {
        fn f(&self);
        fn foo() -> i32;
    }

    trait Bar {
        fn f(&self);
    }

    struct Baz;

    impl Baz {
        fn foo() -> i32 {
            println!("foo in Baz");
            3
        }
    }

    impl Foo for Baz {
        fn f(&self) {
            println!("Baz 's impl of Foo");
        }

        fn foo() -> i32 {
            println!("Foo in Baz");
            3
        }
    }

    impl Bar for Baz {
        fn f(&self) {
            println!("Baz's impl of Bar");
        }
    }

    #[test]
    fn test() {
        let b = Baz;

        Foo::f(&b);
        Bar::f(&b);

        <Baz as Foo>::f(&b);


    }
}
