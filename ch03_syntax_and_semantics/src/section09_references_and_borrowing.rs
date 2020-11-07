#[cfg(test)]
mod tests {

    fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32> , i32) {

        (v2, v1, 423)
    }

    fn sum_vec(v: &Vec<i32>) -> i32 {
        v.iter().fold(0, |a, &b| {a + b})
    }

    fn foo2(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        sum_vec(v1) + sum_vec(v2)
    }

    fn foo3(v: &mut Vec<i32>) {
        v.push(33)
    }

    #[test]
    fn test() {

        let v1 = vec![1, 2, 3];
        {
            let v2 = vec![3, 4];

            let (v1, v2, v) = foo(v2, v1);

            println!("{:?}", v1);

        }

        let v1 = vec![1, 2, 3];

        {
            let v2 = vec![1, 2, 3];

            let v = foo2(&v1, &v2);

            println!("{}", v);
        }

        println!("{:?}", v1);


        // println!("{:?}", v1);

        let v1 = vec![1, 2, 3];
        let v2 = vec![2, 3, 4];

        let sum = foo2(&v1, &v2);

        println!("{}", sum);

        let mut x = 5;
        {
            let y = &mut x;

            *y += 1;
        }

        println!("{}", x);


        let mut x = 5;

        let y = &mut x;
        let mut y = &mut x;
        *y = 20;

        println!("{}", y);

        let mut x = 5;
        let y = &mut x;

        *y += 21;

        let y2 = &mut x;
        // *y2 += *y;

        // let m = *y;
        *y2 = 0;


        println!("{}", x); // borrow 又一次把borrow给借走,看来是这样问题


        let mut x = 5;
        let y = &mut x;
        *y += 1;
        println!("{}", x);
        test_001();
        test_002();
    }

    fn test_001() {
        let mut v = vec![1, 2, 3];

        for i in &v {  // &v 当写成 &v的时候,v已经是不可变的类型
            println!("i is {}", i);
            // v.push(3);
        }

        let y: &i32;
        let x = 5;
        y = &x;

        println!("{}", y);

    }

    fn test_002() {
        let y: &i32;
        let x = 5;
        y = &x;

        println!("{}", y);
    }

    // The Rules
    // First, any borrow must last for a scope no greater than that of the owner. Second, you
    // may have one or other these two kind of borrows, but not both at the tiem:
    //    One or more references (&T) to a resource
    //    Exactly one mutable reference (&mut T)

    // References must not live longer than the resource they refer to. Rust will check the
    // scopes of your references to ensure that this is true.
}
