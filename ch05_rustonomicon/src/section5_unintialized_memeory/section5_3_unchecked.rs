//!
//!
//! # Unchecked Uninitialized Memory
//!
//!
//!
//!
//!


#[cfg(test)]
mod tests {

    use std::mem::{self, MaybeUninit};

    // Size of the array is hard-coded but easy to change (meaning, changing just the constant is sufficient)
    // this means we can't use [a, b, c] syntax to initialize the array, though, as we would have to keep that
    // in sync with `SIZE`!

    const SIZE:usize = 10;

    #[test]
    fn test_001() {

        let x = {
            // Create an uninitialized array of `MaybeUninit`. The `assume_init` is safe because the type we are
            // claiming to have initialized here is a bunch of `MaybeUninit`'s, which do not require initialization.

            let mut sx: [MaybeUninit<Box<u32>>; SIZE] = unsafe {
              MaybeUninit::uninit().assume_init()
            };

            // Dropping a `MaybeUninit` does nothing, Thus using raw pointer assignment instead of
            // `ptr::write` does not cause the old uninitialized value to be dropped.
            // Exception safety is not a concern because Box can't panic

            for i in 0..SIZE {
                sx[i] = MaybeUninit::new(Box::new(i as u32));
            }

            // Everything is initialized. Transmute the array to the initialized type.
            unsafe {
                mem::transmute::<_, [Box<u32>; SIZE]>(sx)
            }

        };
        println!("{:?}", x);


    }

}