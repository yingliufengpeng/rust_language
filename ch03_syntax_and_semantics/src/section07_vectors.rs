#[cfg(test)]
mod tests {

    #[test]
    fn test() {

        let v = vec![1, 2, 3, 4, 5, 6];

        let v = vec![0; 10];
        println!("{:?}", v);

        // It's also important to note that you must index with the usize type

        let j = 0usize;

        let i = 0i32;

        println!("{}", v[j]);
        //println!("{}", v[i]); //

        for i in &v {
            println!("{}", i);  // A reference to v
        }
        let mut v = vec![1, 2, 4];
        for i in &mut v { // A mutable reference to v
            *i = 20;

        }

        for i in v {
            println!("{}", i);  // Take ownership of the vector and its elements
         }

    }
}
