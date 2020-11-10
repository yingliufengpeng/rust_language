#[cfg(test)]
mod tests {

    // The following are all undefined behaviors in Rust, and must be
    // avoided, even when writing unsafe code:
    //   Date races
    //   Dereferencing a NULL/dangling raw pointer
    //   Reads of undef(uninitialized) memory
    //   Breaking the pointer aliasing rules with raw pointers with pointers
    //   &mut T and &T follow LLVM's scoped noalias model, except if the &T contains
    //      an UnsafeCell<U>. Unsafe code must not violate these aliasing guarantees.
    //   Mutating an immutable value/reference without UnsafeCell<U>
    //   Invoking undefined behavior via compiler intrinsics:
    //      Indexing outside of bounds of an object with std::ptr::offset (offset intrinsic)
    //          with the exception of one byte past the end which is permitted.
    //      Using std::ptr::copy_nonoverlapping_memory (memcpy32/memcpy64 intrinsics) on
    //          overlapping buffers
    //   Invalid values in primitive types, even in private fields/locals
    //      NULL/dangling references or boxes
    //      A value other than false(0) or true(1) in a bool
    //      A discriminant(判别式) in an enum not included in its type definition
    //      A value in a char which is a surrogate(代替的) or above char::MAX
    //      Non-Utf-8 byte sequences in a str
    //   Unwinding into Rust from foreign code or unwinding from Rust into foreign coe.



    unsafe fn danger_will_robinson() {
        // Scary sutff...
    }

    trait Show {
        fn show(&self) {
            println!("show");
        }
    }

    impl Show for i32 {

    }

    unsafe trait Scary {
        fn show22(&self);
    }

    unsafe impl Scary for i32 {
        fn show22(&self) {
            println!("kkk");
        }
    }

    #[test]
    fn test() {
        unsafe {
            println!("kkk");
        }

        let m = 2;
        m.show();
        m.show22();

    }
}
