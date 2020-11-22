
//!
//! # Data Races and Race Conditions
//!     Safe Rust guarantees an absence of data races, which are defined as:
//!         two or more threads concurrently accessing a location of memory
//!         one of them is a write
//!         one of them is unsynchronized
//!


#[cfg(test)]
mod tests {

    #[test]
    fn test_002() {
        use std::thread;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let data = vec![1, 2, 3, 4];

        let idx = Arc::new(AtomicUsize::new(0));
        let other_idx = idx.clone();

        // `move` captures other_idx by-value, moving it into this thread
        thread::spawn(move || {
            // It's ok to mutate idx because this value
            // is an atomic, so it can't cause a Data Race.
            other_idx.fetch_add(10, Ordering::SeqCst);
        });

        if idx.load(Ordering::SeqCst) < data.len() {
            unsafe {
                // Incorrectly loading the idx after we did the bounds check.
                // It could have changed. This is a race condition, *and dangerous*
                // because we decided to do `get_unchecked`, which is `unsafe`.
                println!("{}", data.get_unchecked(idx.load(Ordering::SeqCst)));
            }
        }
    }

    #[test]
    fn test_001() {
        use std::thread;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let data = vec![1, 2, 3, 4];
        // Arc so that the memory the AtomicUsize is stored in still exists for
        // the other thread to increment, even if we completely finish executing
        // before it. Rust won't compile the program without it, because of the
        // lifetime requirements of thread::spawn!
        let idx = Arc::new(AtomicUsize::new(0));
        let other_idx = idx.clone();

        // `move` captures other_idx by-value, moving it into this thread
        thread::spawn(move || {
            // It's ok to mutate idx because this value
            // is an atomic, so it can't cause a Data Race.
            other_idx.fetch_add(10, Ordering::SeqCst);
        });

        // Index with the value loaded from the atomic. This is safe because we
        // read the atomic memory only once, and then pass a copy of that value
        // to the Vec's indexing implementation. This indexing will be correctly
        // bounds checked, and there's no chance of the value getting changed
        // in the middle. However our program may panic if the thread we spawned
        // managed to increment before this ran. A race condition because correct
        // program execution (panicking is rarely correct) depends on order of
        // thread execution.
        println!("{}", data[idx.load(Ordering::SeqCst)]);

    }

}