use crate::mod1::mod1_show;

fn show2() {

    mod1_show();

}


#[cfg(test)]
mod tets {
    use crate::mod2::utils::show2;

    #[test]
    fn test_001() {

        show2();
    }
}