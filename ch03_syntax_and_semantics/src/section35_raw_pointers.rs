#[cfg(test)]
mod tests {

    // Here are some things to remember about raw pointers that are
    // different than other pointer types.
    // They:
    //    are not guaranteed to point to valid memory and are not
    //      event guaranteed to be non-NULL(unlink both Box and &);
    //    do not have any automatic clean-up, unlike Box, and so require
    //      manual resource management
    //    are plain-old-data, that is, they don't move ownership, again
    //      unlike Box, hence the Rust compiler cannot protect against bugs
    //      like use-after-free;
    //    lack any form of lifetimes, unlike &, and so the compiler cannot
    //      reason about dangling pointers; and
    //    have no guarantees about aliasing or mutability other than mutation
    //      not being allowed directly through a *const T.


    #[test]
    fn test() {

        let x = 5;
        let raw = &x as *const i32;
        let mut y = 10;
        let raw_mut = &mut y as *mut i32;


        let points_at = unsafe { *raw };
        println!("{}", points_at);

        // Explicit cast
        let i = 1u32;
        let p_imm = &i as *const u32;

        // Implicit coercion
        let mut m = 2u32;
        let p_mut = &mut m as *mut u32;

        unsafe {
            let ref_imm = &*p_imm;
            let ref_mut = &mut * p_mut;

            *ref_mut = 100u32;
        }
        println!("{}", m);
    }
}
