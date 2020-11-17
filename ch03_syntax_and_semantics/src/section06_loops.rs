

#[cfg(test)]
mod tests {

    #[test]
    fn test_loop() {
        let mut i = 0;
        loop {
            println!("Loop forever!!!");


            if i == 10 {
                break;
            }

            i += 1;

        }

    }

    #[test]
    fn test_while() {

        let mut x = 5;
        let mut done = false;

        while !done {
            if x % 5 == 0 {
                done = true;
            }
        }

    }

    #[test]
    fn test_for() {

        for e in 1..2 {
            println!("{}", e);
        }

        let v = vec![1, 2, 3];

        for e in &v {
            println!("{}", e);
        }

        for e in v.iter() {
            println!("{}", e);
        }

        for (index, value) in v.iter().enumerate() {
            println!("index = {} and value = {}", index, value);
        }
    }
}