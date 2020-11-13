#[cfg(test)]
mod test {

    #[test]
    fn test() {

        // 对于rust 宏的学习需要时间去消化

        let f = |x| x;

        let x: i32 = f(3); // 单态化
        // f(3.0);

        println!("{}", x);


    }
}
