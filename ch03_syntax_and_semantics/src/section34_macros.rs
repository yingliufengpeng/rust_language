pub fn increment(x: u32) -> u32 {
    x + 1
}

#[macro_export]
macro_rules! inc {
    ($x:expr) => ( $crate::section34_macros::increment($x) )
}

macro_rules! ind {
    ($x:expr) => ( ::increment($x) )
}


#[cfg(test)]
mod tests {
    use std::fmt::Write;
    use std::string::String;
    use std::fs::File;


    macro_rules! foo2 {
        () => { // outer braces
          { // inner braces

          }
        }
    }

    // The outer braces part of the syntax of macro_rules!. In fact, you can use () or [] instead.
    // They simply delimit the right-hand side as a whole.
    // The inner braces are part of the expanded syntax. Remember, the vec! macro is used in an
    // expression context. To write an expression with multiple statements, including let-bindings,
    // we use a block. If your macro expands to a single expression, you don't need this extra layer
    // of braces

    macro_rules! foo {
        (x => $e:expr) => (println!("mode X: {}", $e));
        (y => $e:expr) => (println!("mode Y: {}", $e));
    }

    macro_rules! o_0 {
        (
            $(
                $x: expr; [ $( $y: expr ),* ]; // 匹配另种类型的items,这两种类型的对象应该如何思考和使用呢?
            )*
        ) => {
            &[ $( $($x + $y), * ), *]
        }
    }

    macro_rules! five_times {
        ($x: expr) =>  (5 * $x);
    }

    macro_rules! my_vec {
        () => (std::vec::Vec::new())
    }

    macro_rules! log {
        ($msg: expr) => {{
            let state: i32 = get_log_state();
            if state > 0 {
                println!("log({}): {}", state, $msg);
            }
        }}
    }
    //
    macro_rules! write_html {

        ($w: expr, ) => {
            println!("叶子节点");
            ();
        };

        ($w: expr, $e: tt) => {
            println!(" 中间节点 e is {}", $e);
            write!($w, "{}", $e)
        };

        ($w: expr, $tag: ident [ $( $inner:tt )* ] $( $rest: tt )* ) => {{  // dispatch路由分发器
            write!($w, "<{}>", stringify!($tag));
            write_html!($w, $( $inner )* );
            write!($w, "</{}>", stringify!($tag));
            write_html!($w, $( $rest )* );  // 这个则是平级结构的写法
        }};

    }
    macro_rules! write_html2 {
        ($w:expr, ) => (());

        ($w:expr, $e:tt) => (write!($w, "{}", $e));

        ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
            write!($w, "<{}>", stringify!($tag));
            write_html2!($w, $($inner)*);
            write!($w, "</{}>", stringify!($tag));
            write_html2!($w, $($rest)*);
        }};
    }

    fn get_log_state() -> i32 {
        4
    }

    macro_rules! b_variable {
        ($v: ident) => (let $v = 3;)
    }

    macro_rules! fn_x {
        () => { fn x2() {}; }
    }

    #[test]
    fn test() {
        let x = vec![1, 2, 3];
        println!("{:?}", x);

        let x = {
            let mut temp_ve = Vec::new();
            temp_ve.push(1);
            temp_ve.push(2);
            temp_ve.push(3);
            temp_ve
        };

        println!("{:?}", x);

        // ( $( $x:expr ), * ) => { ... };
        foo!(x => 3);
        foo! {y => 3}
        ;

        let v0 = o_0!(
            10; [1, 3, 4];
            20; [4, 5, 6];
            30; [7, 8, 9];
        );

        println!("{:?}", v0);

        let c = five_times!(3);
        println!("{}", c);

        let d = vec![1, 2, 3];

        let e: Vec<i32> = my_vec!();
        let state = 30;
        log!("你的消息已经失效了!!!");
        println!("{}", state);

        b_variable!(x);
        println!("{}", x);

        fn_x!();
        x2();

        let mut out = String::new();

        write_html2!(out, );
        write_html!(
            &mut out, html [
                head[title["Rust Guide"]]
                body[h1["Rust is the best programming language!!!"]]
                footer[inner["inner"] outer["outer"]]
            ]
        );

        // write_html!(&mut out, 33);
        //
        println!("{}", out);
    }

    fn foo_2() -> std::io::Result<()> {

        let f =  File::create("foo.txt")?;

        Ok(())

    }

    fn foo_2_translate() -> std::io::Result<()> {

        let f = File::create("foo.txt");

        let f =  match f {
            Ok(f) => f,
            Err(err) => return Err(err),
        };
        Ok(())
    }

    #[test]
    fn test_002() {

        let x = inc!(3);

        if false {

            unreachable!();
        }

        let x = None as Option<i32>;

        match x {
            Some(_) => unreachable!(),
            None => {
                println!("I know x is not None");
            }
        }

    }
}

macro_rules! m1 { () => ( () ) }
// Visible here: `m1`

mod foo {
    // Visible here: `m1`

    #[macro_export]  // 这样的宏,我们是可以导出的,导出之后可以使用
    macro_rules! m2 { () => ( ( ) ) }

    // Visible here: `m1`, `m2`

}


// Visible here: `m1`.

macro_rules! m3 { () => ( () ) }

// Visible here: `m1`, `m3`

#[macro_use]
mod bar {
    // Visible here: `m1`, `m3`.

    macro_rules! m4 { () => (  ()  ) }

    // Visible here: `m1`, `m3`, `m4`.
}

// Visible here `m1`, `m3`, `m4`.








