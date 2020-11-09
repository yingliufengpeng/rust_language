#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let plus_one = |x: i32| x + 1;
        assert_eq!(2, plus_one(1));

        let plus_two = |x| {
          let mut result: i32 = x;
            result += 1;
            result += 1;
            result
        };

        assert_eq!(4, plus_two(2));

        let mut num = 5;
        let mut plus_num = |x: i32| {
            num += 1;
            x + num
        };
        plus_num(3);

        let y = &mut num;

        println!("{}", y);


        let nums = vec![1,2, 3];
        let takes_nums = || nums;  // Closure 已经获得了nums的所有权

        // println!("{:?}", &nums);

        let num = 5;
        let owns_num = |x: i32| num;

        println!("{}", num);

        let num = 120;
        let owns_num = move |x: i32| num;
        println!("{}", num);

        let mut num = 5;
        {
            let mut add_num = |x: i32| num += 5;

            add_num(5);
        }

        println!("{}", num);

        let mut num = 5;
        {
            let mut add_num = move |x: i32| num += 5; // 有move语义,所以就是把所有权给获取,但是i32实现了copy语义,所以复制了一份新的对象给它

            add_num(5);
        }

        println!("{}", num);




        fn call_with_one<F>(some_closure: F) -> i32
            where
                F: Fn(i32) -> i32
        {
            some_closure(1)
        }


        let answer = call_with_one(|x| x + 2);

        println!("{}", answer);


        fn call_with_one2(some_closure: &dyn Fn(i32) -> i32) -> i32 {
            some_closure(1)
        }


        let answer = call_with_one2(&|x| x + 2);

        println!("{}", answer);

        fn call_with_ref<F>(some_closure: F) -> i32
            where F: Fn(&i32) -> i32 {
            let value = 0;
            some_closure(&value)
        }

        let answer = call_with_ref(|x| x + 1);
        println!("{}", answer);


        // fn call_with_ref2<'a, F>(some_closure: F) -> i32
        //     where F: Fn(&'a i32) -> i32
        // {
        //     let value = 3;
        //     some_closure(&value) // borrow checker will complain that value doesn't live long enough.
        // }

        fn call_with_ref3<F>(some_closure:F) -> i32
            where F: for<'a> Fn(&'a i32) -> i32
        {
            let value = 3;
            some_closure(&value)
        }

        let answer = call_with_ref3(|x| x + 3);
        println!("{}", answer);

        fn call_with_one4(some_closure: &dyn Fn(i32) -> i32) -> i32 {
            some_closure(12)
        }

        fn add_one2(i: i32) -> i32 {
            i + 1
        }

        let f = add_one2;

        let answer = call_with_one4(&f);
        println!("{:?}", answer);


        fn factory() -> Box<dyn Fn(i32) -> i32> {
            let num = 5;
            Box::new(move |x| x + num) // move 拷贝num一份给闭包对象,否则num会被析构
        }

        let f = factory();
        let answer = (*f)(3);
        println!("answer is {}", answer);



        // The key to understanding how closures work under the hood is something a bit strange: Using
        // () to call a function, like foo(), is an overloadable operator. From this, everything else
        // clicks into place. In Rust, we use the trait system to overload operators. Call functions is
        // no different. We have three separate traits to overload with:
        // Fn
        // FnMut
        // FnOnce
        // There are a few differences between there traits, but a big one is self: Fn takes &self,
        // FnMut takes &mut self, and FnOnce takes self. This cover three kinds of self via the usual
        // method call syntax. But we've split them up into three traits,rather than having a single
        // one. This give us a large amount of control over what kind of closure we can take.

    }
}
