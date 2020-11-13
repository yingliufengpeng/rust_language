macro_rules! replace_expr {
    ($_t:tt $sub:expr) => { {
        let v: $_t = $sub;
        v
    } };
}


macro_rules! tuple_default {
    ($($tup_tys:ty),*) => {
        {
            (
                $(
                    replace_expr!(
                       ($tup_tys) Default::default()
                    ),
                )*
            )
        }
    };
}

// Here, we are not actually using the matched types. Instead, we throw them away and instead
// replace them with a single, repeated expression. To put it another way, we don't care what
// the types are, only how many there are.

#[cfg(test)]
mod tests {
    #[test]
    fn test_001() {
        let r = i32::default();
        let r = tuple_default!(i32, i32, i64, i32, u64, f32, String, &str);
        println!("{:?}", r);
    }
}