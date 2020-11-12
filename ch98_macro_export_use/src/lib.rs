
#[macro_export]
macro_rules! X {
    () => {Y!();};
}

#[macro_export]
macro_rules! Y {
    () => {}
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
