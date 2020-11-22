
//!
//!
//! # Leaking
//!
//! # Drain
//!
//! # Rc
//!
//! # thread::scoped:JoinGuard
//!     The thread::scoped API intended to allow threads to be spawned that reference data on their
//!         parent's stack without any synchronization over that data by ensuring the parent joins
//!         the thread before any of the shared data goes out of scope.
//!



//
// struct Rc<T> {
//     ptr: *mut RcBox<T>,
// }
//
// struct RcBox<T> {
//     data: T,
//     ref_count: usize,
// }
//
// impl<T> Rc<T> {
//     fn new(data: T) -> Self {
//         unsafe {
//             // Wouldn't it be nice if heap::allocate worked like this?
//             let ptr = heap::allocate::<RcBox<T>>();
//
//             ptr::write(ptr, RcBox {
//                 data: data,
//                 ref_count: 1,
//             });
//             Rc { ptr: ptr }
//         }
//     }
//
//     fn clone(&self) -> Self {
//         unsafe {
//             (*self.ptr).ref_count += 1;
//         }
//         Rc { ptr: self.ptr }
//     }
// }
//
// impl<T> Drop for Rc<T> {
//     fn drop(&mut self) {
//         unsafe {
//             (*self.ptr).ref_count -= 1;
//             if (*self.ptr).ref_count == 0 {
//                 // drop the data and then free it
//                 ptr::read(self.ptr);
//                 heap::deallocate(self.ptr);
//             }
//         }
//     }
// }

// struct Rc2<T> {
//     ptr: *mut RcBox<T>,
// }
//
// struct RcBox2<T> {
//     data: T,
//     ref_count: usize,
// }
//
// impl<T> Rc2<T> {
//     fn new(data: T) -> Self {
//         unsafe {
//             // Wouldn't it be nice if heap::allocate worked like this?
//             let ptr = heap::allocate::<RcBox2<T>>();
//             ptr::write(ptr, RcBox2 {
//                 data: data,
//                 ref_count: 1,
//             });
//             Rc2 { ptr: ptr }
//         }
//     }
//
//     fn clone(&self) -> Self {
//         unsafe {
//             (*self.ptr).ref_count += 1;
//         }
//         Rc2 { ptr: self.ptr }
//     }
// }
//
// impl<T> Drop for Rc<T> {
//     fn drop(&mut self) {
//         unsafe {
//             (*self.ptr).ref_count -= 1;
//             if (*self.ptr).ref_count == 0 {
//                 // drop the data and then free it
//                 ptr::read(self.ptr);
//                 heap::deallocate(self.ptr);
//             }
//         }
//     }
// }


use std::{mem, ptr};
use std::alloc::alloc;
use std::thread;

#[cfg(test)]
mod tests {
    use std::mem;
    use std::thread;

    #[test]
    fn test_001() {
        // let mut data = Box::new(0);
        // {
        //     let guard = thread::scoped(|| {
        //         // This is at best a data race. At worst, it's also a use-after-free.
        //         *data += 1;
        //     });
        //     // Because the guard is forgotten, expiring the loan without blocking this
        //     // thread.
        //     mem::forget(guard);
        // }
// So the Box is dropped here while the scoped thread may or may not be trying
// to access it.

    }

}

#[test]
fn test_002() {
//     let mut data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     {
//         let mut guards = vec![];
//         for x in &mut data {
//             // Move the mutable reference into the closure, and execute
//             // it on a different thread. The closure has a lifetime bound
//             // by the lifetime of the mutable reference `x` we store in it.
//             // The guard that is returned is in turn assigned the lifetime
//             // of the closure, so it also mutably borrows `data` as `x` did.
//             // This means we cannot access `data` until the guard goes away.
//             let guard = thread::scoped(move || {
//                 *x *= 2;
//             });
//             // store the thread's guard for later
//             guards.push(guard);
//         }
//         // All guards are dropped here, forcing the threads to join
//         // (this thread blocks here until the others terminate).
//         // Once the threads join, the borrow expires and the data becomes
//         // accessible again in this thread.
//     }
// // data is definitely mutated here.
}

#[test]
fn test_001() {


    let mut vec = vec![Box::new(0); 40];

    {
        // start draining, vec can no longer be accessed
        let mut drainer = vec.drain(..);

        // pull out two elements and immediately drop them
        drainer.next();
        drainer.next();

        // get rid of drainer, but don't call its destructor
        mem::forget(drainer);
    }

// Oops, vec[0] was dropped, we're reading a pointer into free'd memory!
//     println!("{}", vec[0]);
}