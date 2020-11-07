
mod tests;
mod tests2;

mod xx;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod test_mod {
    use crate::mod1::mod1_utils::Point;

    #[test]
    fn test_002() {

        let p = Point {x: 3, y: 4};

        println!("{:?}", p);

    }
}