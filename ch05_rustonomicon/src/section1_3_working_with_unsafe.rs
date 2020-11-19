
//!
//! # Working with Unsafe
//!
//!
//!
//!
//!

fn index(index: usize, arr:&[u8]) -> Option<u8> {
    if index < arr.len() {
        unsafe  {
            Some(*arr.get_unchecked(index))
        }
    } else {
        None
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_001() {

        let r = vec![1, 2, 3, 4];

        if let Some(v) = index(3, &r) {
            println!("{}", v);
        } else {
            println!("Nothing");
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

