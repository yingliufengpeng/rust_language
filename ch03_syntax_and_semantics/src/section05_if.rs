#[cfg(test)]
mod tests {

    #[test]
    fn test() {

        let x = 5;
        if x == 5 {
            println!("x is five");
        }

        if x == 5 {
            println!("x is five");
        } else if x == 6 {
            println!("x is six");
        } else {
            println!("x is not five or six :(");
        }

        let x = 5;
        let y = if x == 5 {
            20
        } else {
          15
        };




    }
}