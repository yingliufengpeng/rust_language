
// Because macros do not interact with regular item privacy or lookup, any public macro must
// bring with it all other macros that is depends on. This can lead to pollution of the global
// macro namespace, or even conflicts with macros from other crates. It may also cause confusion
// to users who attempt to selectively import macros: they must transitively import all macros,
// including ones that may not be publicly documented.

// A good solution is to conceal what would otherwise be other public macros inside the macro being
// exported. The above example shows how the common as_expr! macro could be moved into publicly
// exported that is using it.

#[macro_use]
mod a {

    #[macro_export]
    macro_rules! foo {
        (@as_expr $e:expr) => {$e};
        ($($tts:tt)*) => {
            foo!(@as_expr $($tts)*)
        }
    }
}

macro_rules! crate_name_util {
    (@as_expr $e:expr) => {$e};
    (@as_item $i:item) => {$i};
    (@count_tts) => {0usize};

}

// Additionally, internal rules will often come before any "bare" rules, to avoid issues with macro_rules!
// incorrectly attempting to parse an internal invocation as something it cannot possibly be ,
//such as an expression.

// If exporting at least one internal macro is unavoidable(e.g. you have many macros that depend on
// a common set of utility rules), you can use this pattern to combine all all internal macros into a
// single uber-macro.


#[cfg(test)]
mod tests {

    #[test]
    fn test_001() {
        let x = foo!( 44);
        println!("{}", x);

        // let x = foo!(4  4); // 死循环的递归,不是好问题

    }

}