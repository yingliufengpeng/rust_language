#[cfg(test)]
mod tests {
    use std::io;
    use std::fs::File;
    use std::io::prelude::*;
    // use std::result::Result;
    use std::io::Result;
    use std::convert::From;

    fn bb() -> Result<i32>  {
        Result::Ok(3)
    }

    fn ab() -> Result<i32> {
        bb()?;

        let r = 20;
        println!("{}", r);
        Ok(r)

    }

    #[test]
    fn test_002() {




    }


    #[test]
    fn test_001() {
        let mut v = vec![1, 2, 3];

        let p = &mut *v;

        println!("{:?}", p);

        let mut v = 20;
        let p = &mut v;
        *p = 20;
        println!("{}", p);
    }
}