

macro_rules! each_tt {
    () => {};
    ($_tt:tt $($rest:tt)*) => {
        println!("_tt is {}", $_tt);
        each_tt!($($rest)*);
      };
}

macro_rules! sing {
    () => {};
    ($tt:tt $($rest:tt)* ) => {
        log_syntax!($tt);
        println!("{:?}", $tt);
        sing!($($rest)*);
     };
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_001() {

        each_tt!("boo" "bar" "baz" "quux");
        trace_macros!(true);
        sing!("boo" "bar" "baz" "quux");


    }
}