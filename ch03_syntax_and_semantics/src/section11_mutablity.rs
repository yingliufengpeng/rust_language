

mod tests {
    use std::sync::Arc;
    use std::cell::RefCell;
    use std::cell::Cell;

    #[test]
    fn test() {

        let mut x = 5;

        let y = &mut x;

        *y = 20;
        println!("{}", y);

        let (mut x, mut y) = (5, 6);

        x = 5;
        y = 20;

        test_001();
    }

    fn test_001() {

        let x = Arc::new(&5);

        let x = Arc::new(5);
        let y = x.clone();

        let z = x;
        // println!("{}", x);
        println!("{}", z);

        let x = RefCell::new(4);

        let mut y = x.borrow_mut();
        // let mut y2 = x.borrow();
        *y = 20;
        println!("{:?}", y);


        let mut point = Point {x: 3, y: 4};
        println!("{:?}", point);
        point.x = 3;

        let point2 = Point2 {x: Cell::new(3), y: Cell::new(54)};

        point2.x.set(33);

        println!("{:?}", point2);


    }

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32
    }

    #[derive(Debug)]
    struct Point2 {
        x: Cell<i32>,
        y: Cell<i32>,
    }


    // Interior vs. Exterior Mutability

}
