
macro_rules! init_array {
    (@accum (0, $_e:expr) -> ($($body:tt)*) ) => {
        init_array!(@as_expr [ $($body), * ]  )  // 最后一步构建[]列表数组结构
    };

    (@accum (1, $e:expr) -> ($($body:tt)*) ) => {
        init_array!(@accum (0, $e) -> ($($body)*  $e))
    };

    (@accum (2, $e:expr) -> ($($body:tt)*) ) => {
        init_array!(@accum (1, $e) -> ($($body)* $e))
    };

    (@accum (3, $e:expr) -> ($($body:tt)*) ) => {
        init_array!(@accum (2, $e) -> ($($body)*  $e))
    };

    (@accum (4, $e:expr) -> ($($body:tt)*) ) => {
        init_array!(@accum (3, $e) -> ($($body)*  $e))
    };

    [$e:expr; $n:tt] => { // 这个是入口点表达式
        {
            let e = $e;
            init_array!(@accum ($n, e.clone()) -> ())  // 从一开始就开始构建结构
        }
    };
    (@as_expr $e:expr) => {$e};


}

macro_rules! init_array2 {

    (@accum 0, $_e:expr) => {};
    (@accum 1, $e:expr) => { $e };
    (@accum 2, $e:expr) => { $e, init_array2!(@accum 1, $e) };
    (@accum 3, $e:expr) => { $e, init_array2!(@accum 2, $e) };

    [$e:expr; $n:tt] => {{
        let e = $e;
        [init_array2!(@accum $n, e)]
    }};

}
// However, this would require each intermediate step to expand to an incomplete expression.
// Even though the intermediate results will never be used outside of a macro context, it is
// still forbidden.




#[cfg(test)]
mod tests {

    #[test]
    fn test_002() {
        // let r = init_array2![0; 2];  // init_array2这个宏是个失败的写法
        // println!("{:?}", r);

    }

    #[test]
    fn test_001() {
        let r:[&str; 4] = init_array!["33"; 4];
        println!("{:?}", r);

        let r: [i32; 4] = init_array![33; 4];
        println!("{:?}", r);
    }

}