#[cfg(test)]
mod tests {

    struct Point {
        x:i32,
        y:i32,
    }

    #[derive(Debug)]
    struct Person {
        name: Option<String>,
    }

    fn test_003() {
        let name = "Steve".to_string();
        let x = Some(Person {name: Some(name)});

        match x {
            Some(Person { name: ref a @ Some(_), .. }) => println!("name is {:?}", a), // 这样的写法会把数据的所有权一并都给带走,这个并不是我们想要的结果
            _ => {},
        }

        let x = OptionalInt::Value(43);

        match x {
            OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
            OptionalInt::Value(..) => println!("Got an int!"),
            OptionalInt::Missing => println!("No such luck"),
        }

        let x = 4;
        let y = false;
        match x {
            4 | 5 if y => println!("yes"),
            _ => println!("no")
        }
    }

    enum OptionalInt {
        Value(i32),
        Missing,
    }


    fn test_002() {

        test_003();

        let x = 5;
        match x {
            ref x => println!("{}", x),
        }
        let mut y = 5;
        match y {
            ref  mut y => println!("{}", y),
        }

        let x = 1;
        match x {
            a @ 1 ... 5 => println!("one through five"),
            _ => println!("anythings"),
        }

        let x = '💅';

        match x {
            'a' ... 'j' => println!("early letter"),
            'k' ... 'z' => println!("late letter"),
            _ => println!("something else")
        }

        let x = 1;
        match x {
            ref e @ 1 ... 5 => println!("got a range element {}", e),
            _ => println!("anything!!!"),
        }
    }

    fn test_001() {
        let origin = Point { x: 0, y: 2};
        match origin {
            Point {x, y} => println!("({}, {})", x, y),
        }

        match origin {
            Point { x: x1, y: y1} => println!("({}, {})", x1, y1),
        }

        match origin {
            Point { x, .. } => println!("x is {}", x),
        }

        let x = 1;
        let c = 'c';
        match c {
            x => println!("x: {} c: {}", x, c),
        }

        println!("x: {}", x); // x introduce new binding

        let x = Some(34);
        match x {
            Some(v) => println!("kkk"),

            _ => println!("0"),
        }

        let (x, _, z) = coordinate();

        let tuple = (5, String::from("five"));
        let (x, _s) = tuple; // Here, tuple is moved, because the String moved;

        let tuple = (5, "five".to_string());
        let (x, _) = tuple; // Here tuple is not moved, as the String was never moved, and u32 is Copy:

        // Here, the String created will be dropped immediately, as it's not bound
        let _ = String::from("mkkk");


        let x = OptionvalTuple::Vaule(5, -2, 3);
        match x {
            OptionvalTuple::Vaule(..) => println!("Got a tuple!"),
            OptionvalTuple::Missing => println!("No such luck"),
        }
    }
    enum OptionvalTuple {
        Vaule(i32, i32, i32),
        Missing,
    }

    fn coordinate() -> (i32, i32, i32) {
        (0, 0, 0)
    }

    #[test]
    fn test() {

        test_001();
        test_002();

        let x = 5;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            4 => println!("four"),
            _ => println!("something else")
        }

        let x = 1;
        match x {
            y => println!("x is {}, y is {}", x, y),
        }
        let x = 20;
        println!("x is {}, y is {}", x, x);
    }
}
