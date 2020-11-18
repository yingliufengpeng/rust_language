//!
//!
//! Background: Send and Sync
//! Concurrency is difficult to reason. In rust, we have a strong,
//! static type system to help us reason about our code. As such,
//! Rust gives us two traits to help us make sense of code that can
//! possibly be concurrent.
//!
//!
//!
//! # Send
//!     The first trait we're going to talk about is Send. When a type
//!         T implements Send, it indicates that something of this type
//!         is able to have ownership transferred safely between treads.
//!
//!     This is import to enforce certain restrictions. For example, if
//!         we have a channel connecting two threads, we would want to be able
//!         to send some data down the channel and to the other thread.
//!         Therefore, we'd ensure that Send was implemented for that type.
//!
//!     In the opposite way, if we were wrapping a library with FFI that
//!         isn't thread-safe, we wouldn't want to implement Send, and
//!         so the compiler will help us enforce that it can't leave the
//!         current thread.
//!
//! # Sync
//!     The second of these traits is called Sync. When a type T implements
//!         Sync, it indicates that something of this type has no possibility
//!         of introducing memory unsafety when used from multiple threads
//!         concurrently through shared references. This implies that types
//!         which don't have interior mutability are inherently Sync, which
//!         includes simple primitive types(like u8) and aggregate types containing
//!         them.
//!     For sharing references across threads, Rust provides a wrapper type
//!         called Arc<T>. Arc<T> implements send and Sync if and only
//!         if T implements both Send and Sync. For example, an object of
//!         type Arc<RefCell<U>> cannot be transferred across threads because
//!         RefCell does not implements Sync, consequently Arc<RefCell<U>> would
//!         not implement Send.
//!
//!     These two traits allow you to use the type system to make strong
//!     guarantees about the properties of your code under concurrency.
//!     Before we demonstrate why, we need to learn how to create a concurrent
//!     Rust program in the first place!
//!
//! # Safe Share Mutable State
//!
//!     Due to Rust's type system, we have a concept that sound like
//!         a lie: "safe shared mutable state." Many programmers agree
//!         that shared mutable state is very, very bad.
//!
//!     Someone once said this:
//!          Shared mutable state is the root of all evil. Most languages
//!             attempt to deal with this problem through the 'mutable' part,
//!             but Rust deals with it by solving the 'shared' part.
//!
//!     The same ownership system that helps prevent using pointers incorrectly
//!         also helps rule out data races, one of the worst kinds of concurrency
//!         bugs.
//!
//!
//! # Channels
//! Here's a version of our code that uses channels for synchronization, rather than waiting
//! for a specific time:
//!



#[cfg(test)]
mod tests {

    use std::thread;
    use std::time::Duration;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::sync::mpsc;

    #[test]
    fn test_006() {
        let handle = thread::spawn(|| {
            panic!("oops!");
        });

        let result = handle.join();

        assert!(result.is_err());
    }

    #[test]
    fn test_005() {
        let data = Arc::new(Mutex::new(0));

        // `tx` is the "transmitter" of "sender"
        // `rx` is the "receiver".
        let (tx, rx) = mpsc::channel();

        for _ in 0..10 {
            let (data, tx) = (data.clone(), tx.clone());

            thread::spawn(move || {
                let mut data = data.lock().unwrap();
                *data += 1;
                tx.send(()).unwrap();
            });

        }

        for _ in 0..10 {
            rx.recv().unwrap();
        }

        println!("{:?}", data);
    }

    #[test]
    fn test_004() {

        let mut data = Arc::new(Mutex::new(vec![1, 2, 3]));
        for i in 0..3 {
            let data = data.clone();
            thread::spawn(move || {
                /// Note that lock method of Mutex has this signature:
                /// fn lock(&self) -> LockResult<MutexGuard<T>>
                /// and because Send is not implemented for MutexGuard<T>,
                /// the guard cannot cross thread boundaries, ensuring thread-locality
                /// of lock acquire and release.
                ///
                let mut data = data.lock().unwrap();
                data[0] += 1;
            });
        }
        thread::sleep(Duration::from_millis(50));

        println!("{:?}", data);
    }

    #[test]
    fn test_003() {
        let x = 1;

        /// This is because by default closures capture variables by reference
        /// and thus the closure only captures a reference to x. This is a problem
        /// , because the thread may outlive the scope of x, leading to a
        /// a dangling pointer.
        ///
        /// To fix this, we use a move closure as mentioned in the error message
        /// . move closures are explained in depth here; basically they move
        /// variables from their environment into themselves.
        ///
        /// Many languages have the ability to execute threads, but it's wildly
        /// unsafe. There are entire books about how to prevent errors that
        /// occur from shared mutable state. Rust help out with its type system
        /// here as well, by preventing data races at compile time. Let's talk
        /// about how you actually share things between threads.
        ///
        ///
        ///

        thread::spawn(move || {
           println!("x is {}", x);
        });
    }

    #[test]
    fn test_002() {
        let handle = thread::spawn(|| {
            "Hello from a thread";
        });

        println!("{:?}", handle.join().unwrap());
    }

    #[test]
    fn test_001() {

        thread::spawn( || {
            println!("Hello from a thread");
        });




    }
}