#[cfg(test)]
mod tests {

    #[test]
    fn test() {
         let x = Some(5);
        println!("{:?}", x);

        fn takes_anything<T>(x: T) {
            // ...
        }

        fn takes_two_of_the_same_things<T>(x: T, y: T) {
            // ...
        }

        fn takes_two_things<T: std::fmt::Display, U: std::fmt::Display>(x: T, y: U) {
            // ...
            println!("{}", x);
            println!("{}", y);
        }

        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        impl <T> Point<T> {
            fn swap(&mut self) -> &mut Self  {
                std::mem::swap(&mut self.x, &mut self.y);
                self
            }
        }

        let int_origin = Point {x: 0, y: 0};
        let float_origin = Point {x: 0.0, y: 1.0};



        println!("{:?}", int_origin);
        println!("{:?}", float_origin);


        takes_two_things(3, 4.0);
        takes_two_things(3.3, 4.4);

        let v: Vec<i32> = Vec::new();

        let v2 = Vec::<i32>::new(); // 又一次学会了一种新的招数
        println!("{:?}", v2);
    }
}
