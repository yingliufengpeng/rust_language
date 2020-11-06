
#[cfg(test)]
mod tests {

    fn add_one(x: i32) -> i32 {
        x + 1
    }



    #[test]
    fn test() {
        let mut y = 5;
        let x = (y = 6); // `x` has the value `()`, not `6`

        // diverging functions
        // let x: i32 = diverges();
        // let x: String = diverges();

        // function pointers
        let f: fn(i32) -> i32;

        fn plus_one(i: i32) -> i32 {
            i + 1
        }
        f = plus_one;

        f(3);

    }

    fn diverges() -> ! {
        panic!("This function never returns!");
    }
}