


#[cfg(test)]
mod tests {
    use super::super::*;



    trait HelloWorld {
        fn hello_world();
    }


    #[derive(HelloWorld2)]
    #[HelloWorldName = "the best Pancakes"]
    struct FrenchToast;

    #[derive(HelloWorld2)]
    struct Waffles;

    // #[derive(HelloWorld2)]
    // enum Waffles2 {
    //     A,
    //     B,
    // }


    #[test]
    fn test_002() {
        FrenchToast::hello_world();
    }

    fn ok() -> i32 {

        if let Some(a) = Some(&3) {
             *a
        } else {
            panic!("kkk");
        }


    }

    #[test]
    fn test_003() {
        ok();
    }

}