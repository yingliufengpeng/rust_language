
//!
//! There are three broad classes of things that are relevant here: iterators, iterator adaptors, and
//! consumers. Here's some definitions:
//!     iterators give you a sequence of values
//!     iterator adaptors operate on an iterator, producing a new iterator with a different output sequence.
//!     consumers operate on an iterator, producing some final set of values.
//!

#[cfg(test)]
mod tests {

    #[test]
    fn test_002() {
        let one_to_one_hundred = (1..101).collect::<Vec<i32>>();
        println!("{:?}", one_to_one_hundred);

        let greater_than_forty_two = (0..100)
            .find(|x| *x > 43);

        println!("{:?}", greater_than_forty_two);

        let sum = (1..4).fold(0, |a, e| a + e);
        println!("{}", sum);

        for i in (1..).take(100) {
            println!("{}", i);
        }
    }

    #[test]
    fn test_001() {
        for x in 0..10 {
            println!("{}", x);
        }


        let nums = vec![1, 2, 3];
        for i in 0..nums.len() {
            println!("{}", nums[i]);
        }

        let nums = vec![1, 2 , 3];
        for num in &nums {
            println!("{}", num);
        }

        for i in (1..10).filter(|&x| x % 2 == 0) {
            println!("{}", i);
        }

        let r = (1..)
            .filter(|&x| x % 2 == 0)
            .filter(|&x| x % 3 == 0)
            .take(10)
            .collect::<Vec<i32>>();

        println!("{:?}", r);
    }
}