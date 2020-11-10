#[cfg(test)]
mod tests {

    use std::ops::Add;
    use std::ops::Mul;

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Add<i32> for Point {
        type Output = Point;

        fn add(self, rhs: i32) -> Self::Output {
            Self {
                x: self.x + rhs,
                y: self.y + rhs,
            }
        }
    }

    trait HasArea<T> {
        fn area(&self) -> T;
    }

    struct Square<T> {
        x: T,
        y: T,
        side: T,
    }

    impl <T> HasArea<T> for Square<T>
        where T: Mul<Output=T> + Copy
    {
        fn area(&self) -> T {
            self.side * self.side
        }
    }

    #[test]
    fn test() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 2, y: 3 };

        let p3 = p1 + p2;
        println!("{:?}", p3);

        let p4 = p3 + 2;
        println!("{:?}", p4);

        let s = Square {
            x: 0.66,
            y: 0.77,
            side: 12.04,
        };

        println!("{}", s.area());

    }
}
