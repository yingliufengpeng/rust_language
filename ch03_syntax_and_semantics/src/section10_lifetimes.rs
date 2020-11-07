
static FOO: i32 = 5;
const  X: &'static str = "kkk";


#[cfg(test)]
mod tests {

    struct Foo<'a> {
        x: &'a i32,
    }

    // Here are the three rules:
    // Each elided lifetime in a function's arguments becomes a distinct lifetime parameter

    // If there is exactly one input lifetime, elided or not, that lifetime is assigned to all
    // elided lifetimes in the return values of that function.

    // If there are multiple input lifetimes, but one of them is &self or &mut self, the lifetime
    // of self is assigned to all elided output lifetimes.


    #[test]
    fn test() {
        let x: &'static str = "Hello, World";

        test_001();

        test_002();

        test_003();

        let x: &'static str = "hello";


    }

    fn test_001() {
        let r;
        {
            let i = 1;
            r = &i;
        }

        // println!("{}", r);
    }

    fn _skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
        line
    }

    fn test_002() {

        let line = "lang:en=Hello World";
        let lang = "en";

        let v;
        {
            let p = format!("lang: {} = ", lang);
            v = _skip_prefix(line, p.as_str());


        }

        println!("{}", v);
    }

    fn test_003() {

        let y = &5;
        let f = Foo {x: y};

        println!("{}", f.x);
        println!("{}", f.x());

    }

    impl <'a> Foo<'a> {
        fn x(&self) -> &'a i32 {
            self.x
        }
    }
}
