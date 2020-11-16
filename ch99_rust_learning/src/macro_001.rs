macro_rules! tt_test {

    ( $t:tt ) => {
        0
    }

}

macro_rules! tt_test2 {
    ($($t:tt)*  ) => {0};
}

macro_rules! tt_test3 {

    (
        ( $($t:tt)* )
     ) => {0};
}

macro_rules! tt_test4 {
    ( (), () => ($($c:tt)*) ) => {
        {
            $(
                println!("{}", stringify!($c));
            )*

            1

        }
    };

    (
        ($($a:tt)*), () => ($($t:tt)*)
    ) => {
        tt_test4!(
            () , () => ($($a)* $($t)* )
        )
    };

    (
        () , ($($b:tt)*) => ($($t:tt)*)
    ) => {
         tt_test4!(
            () , () => ($($b)* $($t)* )
        )
    };

    (
        ($h1:tt $($h:tt)* ), ($t1:tt $($t:tt)* ) => ( $($k:tt)* )
    ) => {
        tt_test4!(
            ( $($h)* ), ($($t)* ) => ( $h1 $t1 $($k)* )

        )
    };
}

#[cfg(test)]
mod tests {
    use std::mem::take;

    #[test]
    fn test_001() {
        let r = tt_test!(3);
        println!("{}", r);
        let r = tt_test!((() ()));
        println!("{}", r);

        let r = tt_test2!(a b ; ; ; ; ; ; ; ; ; JA A A A A  A AADd d d d d d d);
        println!("{}", r);

        let r = tt_test3!((aa bb ));
        println!("{}", r);

        let r = tt_test4!(
          (  fsfsddf sfsfdsdfds fssf s fsf sdfsfsfdsf a a v v   ) , ( v v v a dd d  D F   sfsf sf sf sdfsd fsd fs fdds fs dfs dfsdf  ) => (a b c d e f )
        );
        println!("{}", r);
        println!("Begin...");
        let r = tt_test4!(
            (A B) , (c d) => (M M M M )
        );

        println!("{}", r);


    }
}