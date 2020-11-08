#[cfg(test)]
mod tests {

    #[test]
    fn test() {

        let s = "hello, world"; // `hello, world` it's type is &'static str

        let s = "foo\
        bar";

        assert_eq!("foobar", s);

        let s = "foo
        bar";

        assert_eq!("foo\n        bar", s);

        fn takes_slice(slice: &str) {
            println!("Got: {}", slice);
        }

        let s = "Hello".to_string();
        takes_slice(&s);
        takes_slice(&&&&&&"kkk");
        let s = "kkk";

        let hachiko = "忠犬ハチ公";
        for b in hachiko.as_bytes() {
            println!("{}", b);
        }

        for c in hachiko.chars() {
            println!("{}", c);
        }

        let hello = "Hello ".to_string();
        let world = "world";
        let hello_world = hello + world;

        let world = "World".to_string();
        let hello_world2 = hello_world + &world;

    }
}
