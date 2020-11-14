

macro_rules! mixed_rules {

    () => {};

    (trace $name:ident; $($tail:tt)*) => { {
        println!(concat!(stringify!($name), " = {:?}"), $name);
        mixed_rules!($($tail)*);
    }};

    (trace $name:ident = $init:expr; $($tail:tt)* ) => {{
        let $name = $init;
        println!(concat!(stringify!($name), " = {:?}"), $name);
        mixed_rules!($($tail)*);

    }};
}

macro_rules! KK {
    ($name:ident) => {
       {
            println!(concat!(stringify!($name), "  xxx   ", "end"));
       }
    }
}

// It is important, however, to keep the macro recursion limit in mid,
// macro_rules! does not have any form of tail recursion or optimisation. It is
// recommended that, when writing a TT muncher, you make reasonable efforts to
// keep recursion as limited as possible.

#[cfg(test)]
mod tests {

    #[test]
    fn test_001() {
        let x = 44;
        let y = 40;
        mixed_rules!(trace x ; trace y; trace z = 30;);

    }

}