
macro_rules! vec1 {

    ( $($e:expr), * ) => {{
        let mut vec = Vec::new();
        $(
            vec.push($e);
        )*

        vec
    }};
}

struct Foo {
    bar: usize,
}




#[cfg(test)]
mod tests {

    #[test]
    fn test_001() {

        let v = vec1![1, 2, 3];

        println!("{:?}", v);
    }
}