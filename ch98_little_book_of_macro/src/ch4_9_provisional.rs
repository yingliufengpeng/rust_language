// abacus(算盘)

macro_rules! abacus {
    (-) => {-1};
    (+) => {1};
    ($($moves:tt)*) => {
        0  $(+ abacus!($moves) ) * // 这里面的+号则是对于这些数据的加法的操作
    };
}

// In this case, the operations become slightly more complicated; increment and decrement
// effectively reverse their usual meanings when the counter is negative. To whit given + and
// - for the positive and negative tokens respectively, the operations change to:
//      Increment by one:
//          match (), substitute (+)
//          match (- $($count:tt)* ), substitute ( $($count)*)
//          match ($($count:tt)+). substitute (+ $($count)+)
//      Decrement by one:
//          match (), substitute (-)
//          match (+ $($count:tt)* ), substitute ( $($count)*)
//          match ($($count:tt)+). substitute (- $($count)+)
//  Compare to 0: match ()
//  Compare to +1: match (+)
//  Compare to -1: match (-)
//  Compare to +2: match (++)
//  Compare to -2: match (--)
//  (and so on...)

macro_rules! abacus2 {
    ( (- $($moves:tt)*) -> (+ $($count:tt)*) ) =>  {
        abacus2!( ($($moves)*) -> ($($count)*)  )
    };

    ( (+ $($moves:tt)*) -> (- $($count:tt)*) ) =>  {
        abacus2!( ($($moves)*) -> ($($count)*)  )
    };

    ( (- $($moves:tt)*) -> ($($count:tt)*) ) =>  {
        abacus2!( ($($moves)*) -> (- $($count)*)  )
    };

    ( (+ $($moves:tt)*) -> ($($count:tt)*) ) =>  {
        abacus2!( ($($moves)*) -> (+ $($count)*)  )
    };


    // Check if the final result is zero
    (() -> ()) => {true};
    (() -> ($($count:tt)*)) => {false};

}


macro_rules! abacu3 {
    ( (- $($moves:tt)*) -> (+ $($count:tt)*) ) =>  {
        abacu3!( ($($moves)*) -> ($($count)*)  )
    };

    ( (+ $($moves:tt)*) -> (- $($count:tt)*) ) =>  {
        abacu3!( ($($moves)*) -> ($($count)*)  )
    };

    ( (- $($moves:tt)*) -> ($($count:tt)*) ) =>  {
        abacu3!( ($($moves)*) -> (- $($count)*)  )
    };

    ( (+ $($moves:tt)*) -> ($($count:tt)*) ) =>  {
        abacu3!( ($($moves)*) -> (+ $($count)*)  )
    };


    // If you want to extract the actual value of the counter,
    // this can be done using a regular counter macro. For example
    // above, the terminal rules can be replaced with following:

    (() -> ()) => {0};

    (() -> (- $($count:tt)*)) => {
        {(-1i32) $(- replace_expr!($count 1i32))*}
    };

    (() -> (+ $($count:tt)*)) => {
        {(1i32) $(+ replace_expr!($count 1i32))*}
    };

}

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {$sub};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_001() {
        let r = abacus!(+ + + + + + );
        println!("{}", r);

        let r = (0, 1, 2, 4, );
        println!("{:?}", r);


        let r = abacus2!((++---+) -> ());
        println!("{}", r);

        let r = abacu3!((+--------+++++) -> ());
        println!("{}", r);

    }
}