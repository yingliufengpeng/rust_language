//  Counting things in a macro is s surprisingly tricky task. The simplest
// way is to use replacement with a repetition match.


// Repetition with replacement
// Recursion
// Slice length
// Enum counting

macro_rules! replace_expr {
    ($_t:tt, $sub:expr) => {$sub};
}

macro_rules! count_tts {
    ( $($tts:tt)* ) => { 0usize $(+ replace_expr!($tts, 1usize))*  }
}

// The compiler must parse this into an AST, which will produce what is effectively
// a perfectly unbalanced binary tree 500+ levels deep.

// This works, but will trivially exceed the recursion limit. Unlike the repetition
// approach, you can extend the input size by matching multiple tokens at once.

macro_rules! count_tts2 {
    ($_a:tt $_b:tt $_c:tt $_d:tt $_e:tt
     $_f:tt $_g:tt $_h:tt $_i:tt $_j:tt
     $_k:tt $_l:tt $_m:tt $_n:tt $_o:tt
     $_p:tt $_q:tt $_r:tt $_s:tt $_t:tt
     $($tail:tt)*)
        => {20usize + count_tts2!($($tail)*)};
    ($_a:tt $_b:tt $_c:tt $_d:tt $_e:tt
     $_f:tt $_g:tt $_h:tt $_i:tt $_j:tt
     $($tail:tt)*)
        => {10usize + count_tts2!($($tail)*)};
    ($_a:tt $_b:tt $_c:tt $_d:tt $_e:tt
     $($tail:tt)*)
        => {5usize + count_tts2!($($tail)*)};
    ($_a:tt
     $($tail:tt)*)
        => {1usize + count_tts2!($($tail)*)};
    () => {0usize};
}

macro_rules! count_tts3 {
    () => {0usize};

    ($head:tt $($tail:tt)*) => { 1usize + count_tts3!($($tail)*)  }
}


macro_rules! count_tts4 {
    ($($tts:tt)*) => { <[()]>::len( &[$(replace_expr!($tts, ())), *] )};
}


// This method does have two drawbacks. First,as implied above, it can only
// count valid identifiers(which are also not keywords), and it does not allow
// those identifiers to repeat.

// Secondly, this approach is not hygienic, meaning that if whatever identifier
// you use in place of __CountIdentsLsat is provided as input, the macro will
// fail due to the duplicate variants in the enum.


macro_rules! count_idents {
    ($($idents:ident),* $(,)*) => {
        {
            #[allow(dead_code, non_camel_case_types)]
            enum Idents { $($idents,)* __CountIdentsLast }
            const COUNT: u32 = Idents::__CountIdentsLast as u32;
            COUNT
        }
    };
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_002() {
        let r = count_idents!{
            A, B, C, D, E, F, G,
            H, I, J, K, M, N, O,
            P, Q, R, S, T, U, V,
            W, X, Y, Z,
        };
        println!("{}", r);
    }

    #[test]
    fn test_001() {
        let r = count_tts![1 2 3 4 5 6 7 8 9];
        println!("r is {}", r);

        let r = count_tts2!(a);
        println!("{}", r);

        let r = count_tts2!(a b c d e);
        println!("{}", r);

        let r = count_tts3!(a b c d e f g h i j k m n );
        println!("{}", r);

        let r = count_tts!(
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,

            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,

            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,

            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,

            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,

            // Repetition breaks somewhere after this
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,

            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,
        );
        println!("{}", r);

        let r = count_tts4!(
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,

            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,

            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,

            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,

            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,

            // Repetition breaks somewhere after this
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,

            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,
            ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,, ,,,,,,,,,,
        );
        println!("{}", r);
    }
}