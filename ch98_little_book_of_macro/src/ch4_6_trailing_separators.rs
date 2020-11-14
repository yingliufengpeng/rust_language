
macro_rules! match_exprs {
    ($($exprs:expr),*, $(,)*) => {0};
}

macro_rules! match_commas {
    ($(,)*) => {0};
}

macro_rules! match_commas2 {
    ($($e:expr,)*) => {0};
}

macro_rules! match_commas3 {
    ( $($e:expr) , * ) => {0};
}


// There are various places in the Rust grammar where trailing commas are permitted. The
// two common ways of matching(for example) a list of expressions $($exprs:expr),* and
// $($exprs:expr,)* can deal with ether no trailing comma or a trailing comma, but not both


#[cfg(test)]
mod tests {

    #[test]
    fn test_001() {
        let r = match_commas!(,,);
        println!("{}", r);

        let r = match_commas2!(3, 4,);
        println!("{}", r);

        let r = match_commas2!(3 ,4, );
        println!("{}", r);


        let r = match_commas3!(3, 4);
        println!("{}", r);

        let r = match_commas3!(3,43);
        println!("{}", r);

        let r = match_exprs!(3, 4, 5, 45, , , , );
        println!("{}", r);

    }

}