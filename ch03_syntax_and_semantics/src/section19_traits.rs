#[cfg(test)]
mod tests {
    // use std::io::Write;

    use std::io::Write;
    use std::fmt::Debug;

    trait ConvertTo<Output> {
        fn convert(&self) -> Output;
    }

    impl ConvertTo<i64> for i32 {
        fn convert(&self) -> i64 {
            *self as i64
        }
    }

    // Can be called with T == i32
    fn convert_t_to_i64<T: ConvertTo<i64>>(x: T) -> i64 {
        x.convert()
    }

    fn convert_i32_to_t<T>(x: i32) -> T
        where i32: ConvertTo<T>
    {
        x.convert()
    }

    trait Foo {
        fn is_valid(&self) -> bool;

        fn is_invalid(&self) -> bool {
            !self.is_valid()
        }
    }

    struct UseDefault;

    impl Foo for UseDefault {
        fn is_valid(&self) -> bool {
            println!("Called UseDefault. isvalid");
            true
        }
    }

    struct OverrideDefault;

    impl Foo for OverrideDefault {
        fn is_valid(&self) -> bool {
            println!("Called OverrideDefault.is_valid");
            true
        }

        fn is_invalid(&self) -> bool {
            println!("Called OverrideDefault.is_invalid");
            true
        }
    }

    fn test_004() {

        trait Foo {
            fn foo(&self);
        }

        trait FooBar: Foo {
            fn foobar(&self);
        }

        struct Baz;

        impl Foo for Baz {
            fn foo(&self) {
                unimplemented!()
            }
        }

        impl FooBar for Baz {
            fn foobar(&self) {
                unimplemented!()
            }
        }

    }



    fn test_003() {
        println!("begin...");
        test_004();
        let m = convert_i32_to_t::<i64>(3);
        println!("{}", m);
        println!("end...");

        let default = UseDefault;
        assert!(!default.is_invalid());

        let over = OverrideDefault;
        assert!(over.is_invalid())
    }

    fn test_002() {
        test_003();

        fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
            x.clone();
            y.clone();
            println!("{:?}", y);
        }

        fn bar<T, K>(x: T, y: K)
            where T: Clone,
                  K: Clone + Debug {
            x.clone();
            y.clone();
            println!("{:?}", y);
        }

        foo(2, 3);
        foo("kk", 4);

        bar(2, 3);
        bar(2.0, 3.0);
    }

    trait HasArea {
        fn area(&self) -> f64;
    }

    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl HasArea for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    struct Square {
        x: f64,
        y: f64,
        side: f64,
    }

    impl HasArea for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    fn print_area<T: HasArea>(shape: T) {
        println!("This shape has an area of {}", shape.area());
    }

    struct Rectangle<T> {
        x: T,
        y: T,
        width: T,
        height: T,
    }

    impl<T: PartialEq> Rectangle<T> {
        fn is_square(&self) -> bool {
            self.width == self.height
        }
    }

    #[test]
    fn test() {
        let c = Circle {
            x: 0.0f64,
            y: 0.0f64,
            radius: 1.0f64,
        };

        let s = Square {
            x: 0.0f64,
            y: 0.0f64,
            side: 1.0f64,
        };

        print_area(c);
        print_area(s);


        test_001();
        test_002();

        // This may seem like the Wild West, but there are two restrictions around implementing
        // traits that prevent this from getting out of hand. The first is that if the trait isn't
        // defined in your scope, it doesn't apply.

        let mut f = std::fs::File::create("foo.txt")
            .expect("Couldn't create foo.txt");

        let buf = b"whatever";


        // This means that even if someone does something bad like add methods to i32, it
        // won't affect you unless you use that trait.
        let result = f.write(buf).expect("有错误发生");

        // One last thing about traits, generic functions with a trait bound use ` monomorphization`,
        // they are statically dispatched.

        fn foo<T: Clone + Debug>(x: T) {
            println!("Not Error is printing!!!");
            x.clone();
        }

        foo(3);
        foo(4.0);
    }

    trait ApproxEqual {
        fn approx_equal(&self, other: &Self) -> bool;
    }

    impl ApproxEqual for f32 {
        fn approx_equal(&self, other: &Self) -> bool {
            (self - other).abs() <= std::f32::EPSILON
        }
    }

    fn test_001() {
        println!("{}", (1.0f32).approx_equal(&0.99999999f32));
    }
}
