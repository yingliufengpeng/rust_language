#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let some_v = Some(4);
        // if some_v.is_none() {
        //     println!("ok");
        //     let x = some_v.unwrap();
        //
        // }


        if let Some(v) = some_v {
            println!("{}", v);
        } else {
            println!("something else!!!");
        }


        let mut v = vec![1, 3, 5, 7, 11];
        loop {

            match v.pop() {
                Some(x) => println!("x is {}", x),
                _ => break,
            }
        }

        let mut v = vec![1, 2, 3, 4, 5];

        while let Some(v) = v.pop() {
            println!("v is {}", v);
        }
    }
}
