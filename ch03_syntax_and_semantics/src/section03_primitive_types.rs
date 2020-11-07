

#[cfg(test)]
mod tests {

    #[test]
    fn test() {

        let x = true;
        let y: bool = false;
        let x = 'x';
        let two_hearts = '💕';
        let x = 42;
        let y = 1.0;

        // Rust also provides types whose particular size depends on the underlying machine architecture.
        let a = [1, 2, 3];
        let mut m = [1, 2, 3];
        m[0] = 200;

        let a = [0; 20];
        println!("{:?}", a);

        let names = ["Gradon", "Brain", "Niko"];
        let n1 = names[1];

        // A `slice` is a reference to (or "view") another data structure.

        let a = [0, 1, 2, 3, 4];
        let complete = &a[..];
        let middle = &a[1..4];

        // Slice have type &[T]
        // A tuple is an ordered list of fixed size.

        let x = (1, "hello");

        println!("{:?}", x);

        let x: (i32, &str) = (1, "hello");


        let mut x = (1, 2);
        x.0 = 10;
        println!("{:?}", x);

        let (x, y, z) = (1, 2, 3);

        let x = (0, );
        let y = (0);


        // Functions also have a type
        fn foo(x: i32) -> i32 {x}

        let x: fn(i32) -> i32 = foo;


    }
}