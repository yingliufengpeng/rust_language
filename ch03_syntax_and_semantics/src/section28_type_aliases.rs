#[cfg(test)]
mod tests {
    use std::result;
    type Name = String;
    type Num = i32;

    enum ConcreteError {
        Foo,
        Bar,
    }

    type Result<T> = result::Result<T, ConcreteError>;

    #[test]
    fn test() {

       let x: Name = "kkk".to_string();

        println!("{}", x);


        let x = 3;
        let y: Num = 4;

        println!("{}", x == y);
    }
}
