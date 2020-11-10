#[cfg(test)]
mod tests {
    use std::mem;

    // The most common case of coercion is removing mutability from a reference
    // &mut T fo &T

    // An analogous conversion is to remove mutability from raw pointer
    // *mut T to *const T


    // Reference can also be coerced to raw ponters:
    // &T to *const T
    // &mut T to *mut T

    // Custom coercions may be defined using Deref
    // Coercion is transitive

    enum Color {
        R,
        G,
        B

    }

    #[test]
    fn test() {
        let x = 5;
        let y = x as i64;

        let z = Color::G as i32;
        println!("{}", z);

        let one = true as u8;
        println!("{}", one);
        let zero = false as u8;
        println!("{}", zero);

        // Pointer casts
        let a = 300 as *const char; // `a` is a pointer to location 300;

        let b = a as u32;

        println!("{}", b);


        unsafe {
            let a = [1u8, 1u8, 0u8, 0u8];
            let b = mem::transmute::<[u8; 4], u32>(a);
            println!("{}", b);

            // Or, more concisely;
            let c: u32 = mem::transmute(a);
            println!("{}", c);
        }



    }
}
