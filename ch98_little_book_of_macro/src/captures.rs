// Captures are written as dollar($) followed by an identifier, a colon(:), and fianlly
// the kind of capture, which must be one of the following:
//  item: an item, like a function, struct, module, etc.
//  block: a block(i.e. a block of statements and/or an expression, surrounded by braces)
// stmt: a statement
// pat: a pattern
// expr: an expression
// ty: a type
// ident: an identifier
// path: a path(e.g. foo, ::std::mem::replace, transmute::<_, int>, ...)
// meta: a meta item; the things that go inside #[..] and #![...] attributes
// tt: a single token tree

// Repetitions
// Patterns can contain repetitions, These allow a sequence of tokens to be matched.
// These have the general form $(...) sep rep.
//      $ is literal dollar token.
//      (...) is the paren-grouped pattern being repeated
//      sep is an optional separator token. Common examples are, and ;
//      rep is the required repeat control. Current, this can be either *(indicating zero
//          or more repeats) or +(indicating one or more repeats). You cannot write "zero or
//          one" or any other more specific counts or ranges


// To defend against future syntax change altering the interpretations of macro input,
// macro_rules! restricts what can follow various captures. The complete list, as of Rust
// 1.3 is as follows:

//      item: anything
//      block: anything
//      stmt: => , ;
//      pat: => , = if in
//      expr: => , ;
//      ident: anything
//      path: , => : =, > ; as
//      meta: anything
//      tt: anything

// Additionally, macro_rule! generally forbids a repetition to be followed by another
// repetition, even if the contents do not conflict.



// Hygiene works by attaching an invisible "syntax context" value to all identifiers.
// When two identifiers are compared, both the identifiers' textural names and syntax
// contexts must be identical for two to be considered equal.


#[cfg(test)]
mod tests {
    macro_rules! one_expression {
        ( $e:expr) => { $e };
    }

    macro_rules! times_five {
        ( $e:expr) => ( 5 * $e);
    }

    macro_rules! multiply_add {
        ($a:expr, $b:expr, $c:expr) => ($a * ($b * $c));
    }

    macro_rules! vec_strs {
        // Start a repetition:
        (
            // Each repeat must contain an expression ...
            $($e:expr)

            // ... separated by commas...
            ,
            // ... zero or more times
            *
        ) => {
            // Enclose the expansion in a block so that we can use
            // multiple statements
            {
                let mut v = Vec::new();

                 // Start a repetition
                 $(
                   // Each repeat will contain the following statement, with
                   // $e replaced with the corresponding expression.

                   v.push(format!("{}", $e));
                 )*
                 v
            }


        }


    }

    macro_rules! dead_rule {
        ($e:expr) => { () };
        ($i:ident +) => {() };
    }

    macro_rules! capture_expr_then_stringfy {
        ($e:expr) => {
           stringify!($e)
        };
    }

    macro_rules! capture_then_match_tokens {
        ($e:expr) => { match_tokens!($e) };
    }

    macro_rules! match_tokens {
        ($a:tt + $b:tt) => {"Got an addition"};
        ($i:ident) => {"Got an identifier"};
        ($($other:tt)*) => {"Got something else"};
    }

    macro_rules! capture_then_what_is {
        (#[$m:meta]) => {what_is!(#[$m])};
    }

    macro_rules! what_is {
        (#[no_mangle]) => {"no_mangle attribute"};
        (#[inline]) => {"inline attribute"};
        ($($tts:tt)*) => {concat!("something else (", stringify!($($tts)*), ")")};
    }

    macro_rules! trace {

        (trace $name:ident; $($tail:tt)*) => {{

            println!("trace {}", $name );

            $(
                println!("tail is {:?}", $tail);
            )*
        }}
    }

    macro_rules! using_a {
        ($e:expr) => {
            let a = 42;
            $e
        };
    }

    // That said, tokens that were substituted into the expanded output retain their
    // origin syntax context(by virtue of(凭借)having been provided to the macro as opposed
    // to being part of the macro self).
    macro_rules! using_ab {
        ($a:ident, $e:expr) => {
          {
            let $a = 42;
            $e
          }
        };
    }

    #[test]
    fn test_004() {
        let res = using_ab!(a, a / 4);
        println!("kk {}", res);
    }

    #[test]
    fn test_003() {
        let kkk = "kkk";
        let m = 10;
        let n = 20;
        trace!(trace kkk;  (4 + 4) (8)  (9) (m + n) (kkk)  3 kkk m  n {m} [120, 3, 4, ]);

        println!(
            "{}\n{}\n{}\n{}",
            what_is!(#[no_mangle]),
            what_is!(#[inline]),
            capture_then_what_is!(#[no_mangle]),
            capture_then_what_is!(#[inline]),
        );
    }

    #[test]
    fn test_002() {
        println!("{}\n{}\n{}\n{}\n",
                 match_tokens!((caravan)),
                 match_tokens!(3 + 45),
                 match_tokens!(5),
                 match_tokens!(a),
        );

        println!("{}\n{}\n{}\n{}\n",
                 capture_then_match_tokens!((caravan)),
                 capture_then_match_tokens!(3 + 45),
                 capture_then_match_tokens!(5),
                 capture_then_match_tokens!(a),
        );
    }

    #[test]
    fn test_001() {
        let x = one_expression!(3);
        println!("{}", x);

        let x = times_five!(4);
        println!("{}", x);

        let x = multiply_add!(1, 2, 3);
        println!("{}", x);

        let x = vec_strs![3, 4, 5, 7, "kk"];
        println!("{:?}", x);

        println!("{:?}", stringify!(   dummy(  2 *   ( 1 +    ( 3 )))));
        println!("{:?}", capture_expr_then_stringfy!(dummy(2 * ( 1 +     ( 3 ))))); // 和little book of macro 这本书的内容提供不同,这两个的结果是是一致的
    }
}
