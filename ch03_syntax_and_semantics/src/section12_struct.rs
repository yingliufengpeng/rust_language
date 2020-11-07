#[cfg(test)]
mod tests {

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    struct PointRef<'a> {
        x: &'a mut i32,
        y: &'a mut i32,
    }

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    #[derive(Debug)]
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    #[test]
    fn test() {
        let mut point = Point {x: 2, y: 4};
        point.x = 44;

        let mut point = point; // `point` is now immutable

        println!("{:?}", point);

        {
            let r = PointRef {
                x: &mut point.x,
                y: &mut point.y
            };

            *r.x = 5;
            *r.y = 55;
        }

        println!("{:?}", point);

        let name = "Peter";
        let age = 27;
        let peter = Person {name, age};

        println!("{:?}", peter);


        let mut point = Point3d {x: 0, y: 0, z: 0};
        point = Point3d {x : 1, ..point};

        println!("{:?}", point);


        let black = Color(0, 0, 0);
        let origin = Point2(0, 0, 0);

        let black_r = black.0;

        let Point2(_, origin_y, origin_z) = origin;




        let length = Inches(10);
        let Inches(integer_length) = length;
        println!("length is {} inches", integer_length);
    }

    struct Color(i32, i32, i32);
    struct Point2(i32, i32, i32);


    // One case when a tuple struct is very useful is when it has only one element. We call this the
    // `newtype` pattern, because it allows you to create a new type that is distinct form its contained
    // value and also expression its own semantic meaning..

    struct Inches(i32);

    // Unit-like structs

    struct Electron {}

    struct Proton;








}
