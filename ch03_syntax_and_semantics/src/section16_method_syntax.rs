#[cfg(test)]
mod tests {

    #[derive(Debug)]
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    struct CircleBuilder {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {

        fn new(x: f64, y: f64, radius: f64) -> Self {
            Self {
                x,
                y,
                radius,
            }
        }

        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
        fn reference(&self) {
            println!("taking self by reference!");
        }

        fn mutable_reference(&mut self) {
            println!("taking self by mutable reference!");
        }

        fn takes_ownership(self) {
            println!("taking ownership of self");
        }

        fn grow(&self, increment: f64) -> Self {
            Self {
                x: self.x,
                y: self.y,
                radius: self.radius + increment,
            }
        }
    }

    impl CircleBuilder {

        fn new() -> Self {
            Self {
                x: 0.0,
                y: 0.0,
                radius: 1.0,
            }
        }

        fn x(&mut self, coordinate: f64) -> &mut Self {
            self.x = coordinate;
            self
        }

        fn y(&mut self, coordinate: f64) -> &mut Self {
            self.y = coordinate;
            self
        }

        fn radius(&mut self, radius: f64) -> &mut Self {
            self.radius = radius;
            self
        }

        fn finalize(&self) -> Circle {
            Circle {
                x: self.x,
                y: self.y,
                radius: self.radius,
            }
        }
    }


    #[test]
    fn test() {
        let c = Circle { x: 0.0, y: 0.0, radius: 2.0};
        println!("{}", c.area());
        let d = c.grow(3.0);
        println!("{}", d.area());

        let c = CircleBuilder::new()
            .x(34.0)
            .y(44.0)
            .finalize();

        println!("{:?}", c);
    }
}
