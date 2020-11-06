


fn hello_world() {
    println!("Hello, World");
}



#[cfg(test)]
mod tests {
    use crate::hello_world;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_hello() {
        hello_world();
    }
}
