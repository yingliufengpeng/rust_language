
// What do other languages do?
// Most languages with a garbage collector heap-allocate by default. This means that every
// value is boxed. There are a number of reasons why this is done, but they're out of scope
// for this tutorial. There are some possible optimizations that don't make it true 100% of
// the time, too. Rather than relying on the stack and Drop to clean up memory, the garbage
// collector deals with heap instead.

// Runtime Efficiency
// Managing the memory for the stack is trivial: The machine increments or decrements a single value,
// the so-called "stack pinter". Managing memory for the heap is non-trivial: heap-allocated memory
// is freed at arbitrary points, and each block of heap-allocated memory can be of arbitrary size, so
// the memory manager must generally work much harder to identify memory for reuse.

// The flexibility of non LIFO-semantics means that in general the compiler cannot automatically infer
// at compile-time where memory should be freed; it has to rely on dynamic protocols, potentially from
// outside the language itself, to drive deallocation(reference counting, as used by Rc<T> and Arc<T>, is one
// example of this)



#[cfg(test)]
mod tests {

    fn foo(i: &i32) {
        let z = 42;
    }

    #[test]
    fn test_003() {
        let x= 5;
        let y = &x;

        foo(y);
    }


    fn italic() {
        let i = 0;
    }

    fn bold() {
        let a = 5;
        let b = 100;
        let c = 1;

        italic();
    }

    #[test]
    fn test_002() {
        let x = Box::new(3);

        let y = 42;
    }


    #[test]
    fn test_001() {
        let x = 42;

        bold();



    }

}