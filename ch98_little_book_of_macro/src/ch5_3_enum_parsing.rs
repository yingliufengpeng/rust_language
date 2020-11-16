enum A {
    B,
}

enum B {
    BB(A),
}

enum C {
    CC(B),
    U,
    W,
    Z,
}


macro_rules! tt_count {
    () => {0usize};

    ($tts:tt  $($tail:tt) *) => {1usize + tt_count!($($tail)*)}
}

macro_rules! tt_expr {
    ( $($tts:tt)* ) => {0}
}

macro_rules! tt_item2 {
    ($($t:item)* $($t2:tt)* ) => { $($t)* }
}

// 该宏主要使用的方法则是使用tt宏来做处理操作,tt宏是以(), [], {}为组合的token树
macro_rules! parse_unitary_variants {
    (@as_expr $e:expr) => {$e};         // 终结组
    (@as_item $($i:item)+) => {$($i)+}; // 终结组
    // 从前两个规则可以看出,callback是一个宏的调用模式,单据诶是不是调用自己,有待理解
    // Exit rules. 很显然,把body里面的东西给情况掉了 箭头之前有两组
    (
        @collect_unitary_variants ($callback:ident ( $($args:tt)* )) ,  ($(,)*) -> ($($var_names:ident,)*)
    ) => {
        parse_unitary_variants! {
            @as_expr
            $callback!{ $($args)* ($($var_names),*) }
        }
    };

    // 箭头之前有两组
    (
        @collect_unitary_variants ($callback:ident { $($args:tt)* }) , ($(,)*) -> ($($var_names:ident,)*)
    ) => {
        parse_unitary_variants! {
            @as_item
            $callback!{ $($args)* ($($var_names),*) }
        }
    };

    // Consume an attribute. 箭头之前也是有两组  只是$fixed 这组不考虑
    (
        @collect_unitary_variants $fixed:tt ,
        (#[$_attr:meta] $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        parse_unitary_variants! {
            @collect_unitary_variants $fixed,
            ($($tail)*) -> ($($var_names)*)
        }
    };

    // Handle a variant, optionally with an with initialiser. 箭头之前也是有两组 只是$fixed 这组不考虑
    (
        @collect_unitary_variants $fixed:tt ,
        ($var:ident $(= $_val:expr)*, $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        parse_unitary_variants! {
            @collect_unitary_variants $fixed,
            ($($tail)*) -> ($($var_names)* $var,)
        }
    };

    // Abort on variant with a payload. 箭头之前也是有两组 只是$fixed 这组不考虑
    (
        @collect_unitary_variants $fixed:tt,
        ($var:ident $_struct:tt, $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        const _error: () = "cannot parse unitary variants from enum with non-unitary variants";
    };

    // Entry rule. e.g. (enum Color {R, G, B } => show (44) ) => 后面是参数的调用
    (   enum $name:ident {  $($body:tt)*  } => $callback:ident $arg:tt ) // $arg 是一个token树
     =>
    {
        parse_unitary_variants! {
            @collect_unitary_variants
            ($callback $arg) , ($($body)*,) -> ()
        }

    };
}

macro_rules! kk {
    (   enum $name:ident {  $($body:tt)*  } => $callback:ident $arg:tt )
        =>
        {
          0

        };

    (@ mm $t:tt) => {
        {
           println!(stringify!($t));
        }

    };

    (@ nn ($t:tt) ) => {
        {
            println!(stringify!($t));
        }
    };
}

macro_rules! tt_count2 {
    () => {0usize};

    ($v:tt  $($tail:tt )*) => {{
        {
            println!(stringify!($v));
            1usize + tt_count2!($($tail)*)
        }
    }}
}

macro_rules! count_w {


    (
        (                   )       =>      ( $($tts:tt )* )

    ) => { tt_count2!($($tts)*) };

    (
        ($head:tt $($tts:tt)*)      =>      ( $($tts2:tt)* )
    ) => {
        count_w!(
                ($($tts)* )         =>      ($head $($tts2)*)
            )
    };


}

// ($callback $arg) token tree 1
// ($($body), * )  token tree2

fn show(x: i32) -> i32 {
    x
}

macro_rules! fun_call {
    ($call:ident $arg:tt) => {$call($arg) }
}

macro_rules! fun_call_early {

    (A B => $call:ident $arg:tt  ) => {$call($arg)};
}

macro_rules! item_test {
    ($tem:item) => {
        {
            // println!("{}", $tem);
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ch5_3_enum_parsing::show;
    enum BB {
        A,
        B,
        C,
    }

    #[test]
    fn test_008() {
        //
        // let r = parse_unitary_variants!(
        //     enum A { R,   }  => tt_item2 (  )
        // );
    }

    #[test]
    fn test_007() {
        //
        // tt_item2!(fn mn() {}  () );
        //
        // tt_item2!( fn mm() {} fn nn() {} );

        let r = item_test!(fn foo() {});
        println!("{}", r);

        let r = item_test!(struct A {});
        println!("{}", r);

        let r = item_test!(struct B;);
        println!("{}", r);

        let r = item_test!(enum Color {R, G, B});
        println!("{}", r);

        let r = item_test!(mod AA;);
        println!("{}", r);


    }


    #[test]
    fn test_006() {

        // kk!(@ mm ((a + b) + (c + d))); // (a + b) 单独的token树的写法
        // kk!(@ nn (( a + b)));
        let r = parse_unitary_variants! {
            enum A {}  => tt_expr  (a b)
        };
        println!("{}", r);
    }

    #[test]
    fn test_005() {
        let r = kk!{
          enum A {}  => show 44
        };

        println!("{}", r);
        // parse_unitary_variants! {
        //
        //     enum A { } => show 55
        // }
    }

    #[test]
    fn test_004() {
        // ( 1 ) 2 = 3 > 4 ( 5 a 6 b 7 c 8 ) 9
        let r = count_w!(
           ( 3 a b c) => ( a b c 2)
        );

        println!("{}", r);

        // parse_unitary_variants!( enum C {  } => show (55) );
        // let r = parse_unitary_variants!( enum C { } => show 55 );
        // println!("{}", r);
    }

    #[test]
    fn test_003() {
        // let r = fun_call_early!(A B => show (44));
        // println!("{}", r);

    }


    #[test]
    fn test_002() {
        let r = tt_count![1 (1 3) ( 3 5) {3 4} [3 4]];
        println!("{}", r);
    }



    #[test]
    fn test_001() {
        let r = 10;
        println!("{}", r);

        // let show = |x: i32| x;

        // parse_unitary_variants! {
        //     enum A {
        //
        //     } =>  show ("44")
        // }

        let r = show(3);

        // let r = fun_call!(show (44 + 44));
        // println!("{}", r);


    }
}