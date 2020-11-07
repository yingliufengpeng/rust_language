#[cfg(test)]
mod tests {

    fn foo() {
        // ownership
        let v = vec![1, 2, 3];

        let v2 = v; // move semantics
    }

    #[test]
    fn test() {

        let v = vec![1, 2, 3];
        let mut v2 = v;

        v2[0] = 0;

        let mut v3 =  v2;

        let v4 = v3;

        // Copy Trait
        let v = 1;
        let v2 = v;

        println!("{}", v);

        // All primitive types implement the Copy trait and their ownership
    }

    fn double(x: i32) -> i32 {
        x * 2
    }

    fn change_truth(x: bool) -> bool {
        !x
    }

    // Of course, if we had to hand ownership back with every function we wrote
    fn foo3(v: Vec<i32>) -> Vec<i32> {
        v
    }

    fn foo2(v1: Vec<i32>, v2: Vec<i32>) ->  (Vec<i32> , Vec<i32>) {
        (v2, v1)
    }


}
