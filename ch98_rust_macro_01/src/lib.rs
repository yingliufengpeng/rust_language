macro_rules! myvec {

    () => { println!("kkk"); };

    // ($arg: ty => $arg2: expr; $arg3: path) => { };

    ($arg1: ty => $arg2: ident  ) => { type $arg2 = $arg1; };
    ($arg1: ty as $arg2: ident  ) => { type $arg2 = $arg1; };

    ($arg1: ty  ) => {type alsou32 = $arg1; };
}

macro_rules! avec {

    ( $x:ident ) => {
        $x += 1;
    };

}

#[macro_export]
macro_rules! vec2 {
    () => {Vec::new()};

    ( $($e:expr), +) => {{
        let mut vec = vec2!();
        $(
            vec.push($e);
        )+
        vec
    }};
}

macro_rules! vec3 {

    () => { Vec::new()  };

    ( $($e: expr), + ; $($x:expr), + ) => {{
        let mut tmp = vec3!();

        $(
            tmp.push($e);

        )+

         $(
            tmp.push($e);

        )+

        $(
            tmp.push($x);
        )+

        tmp

    }};
}

macro_rules! vec4 {

    () => ( Vec::new() );

    ( $($e:expr), + $(,)?) => {{
        let mut vec = vec2!();
        $(
            vec.push($e);
        )+
        vec
    }};
}

macro_rules! vec5 {

    () => ( Vec::new() );

    ( $($e:expr) +) => {{
        let mut vec = vec2!();
        $(
            vec.push($e);
        )+
        vec
    }};
}

trait HasMaxValue {
    fn max_value2(&self) -> Self;
}

macro_rules! max_impl {
    ($t:ty ) => {
         impl $crate::HasMaxValue for $t {
            fn max_value2(&self) -> Self {
                println!("该方法已经实现");
                <$t>::MAX
            }
         }
      } ;

}

max_impl!(i32);
max_impl!(u32);
max_impl!(i64);
max_impl!(u64);

macro_rules! myfoo {

    ($t:tt,  $($e:expr), +    ) => {{
        $(
            println!("{}", $e);
        )+
    }}
}

macro_rules! vec6 {

    () => ( Vec::new() );

    ( $($e:expr) +) => {{
        let mut vec = vec2!();
        $(
            vec.push($e);
        )+
        vec
    }};

    ($e:expr; $count: expr ) => {{
        let count = $count;
        let mut vs = Vec::with_capacity(count);
        let x = $e; // 强制代码执行
        vs.extend(std::iter::repeat(x).take(count));
        vs
    }};

}

macro_rules! vec7 {

    ( $($e: expr), *  ) => {{
        let mut vs = Vec::new();
        $(vs.push($e);)*
        vs
    }};

    ( $($e:expr ,) * ) => {{
        vec7![$($e), *]
    }};

    ($e:expr; $count:expr) => {{

        let count = $count;
        let mut vs = Vec::with_capacity(count);
        vs.resize(count, $e);
        vs

    }};

}

macro_rules! vec8 {

     ( $($e: expr) ; * ) => {{
        const c: usize = $crate::count![@COUNT; $($e),*];
        let mut vs = Vec::with_capacity(c);
        $(vs.push($e);)*
        vs
    }};


}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
  (@COUNT; $($e:expr), * )  => {

     <[()]>::len(&[ $($crate::count![@SUBSTR; $e]), * ])

    };

    (@SUBSTR; $e:expr ) => {
        ()
    };

}




#[cfg(test)]
mod tests {
    use crate::HasMaxValue;

    #[test]
    fn test_003() {
        let v = vec8![1 ; 2 ; 3];
        println!("{:?}", v);

        let v = vec7![1, 2 , 3];
        println!("{:?}", v);
        let v = vec7![1, 2, 3, ];

        let mut x = Some("KKK");
        let v = vec6![x.take().unwrap(); 2];
        println!("{:?}", v);

        myfoo![u32, 0, 0, 0, 0, 0];

        let x = vec5![1 12 3];
        println!("{:?}", x);

        let x = vec4![1, 2, 3, 34, 56, 6, 7,];
        println!("{:?}", x);

        let x = vec4![1, 2, 3, 34, 56, 6, 7];
        println!("{:?}", x);

        let x: Vec<i32> = vec4!();
        println!("{:?}", x);


        let x = 0u32;
        // x.max_value();
        x.max_value2();
    }

    #[test]
    fn test_002() {
        let v = vec2![2, 3, 4, 5];
        println!("{:?}", v);

        let v: Vec<i32> = vec2!();

        let v = vec3![1, 3, 4, 5, 5, 5 ; 3, 4, 5, 6];
        println!("{:?}", v);
    }

    #[test]
    fn test_001() {
        myvec!();
        myvec![];
        myvec! {}

        // myvec! {
        //     u32 => 3; std::path
        // }

        myvec! {
            u32 => u33
        }

        myvec! {
            u64 as u43
        }

        myvec!(u32);
    }
}

