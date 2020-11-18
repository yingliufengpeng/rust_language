
//!
//! Basic pinter types
//!     # Box<T>
//!     # &T and &mut T
//!     # *const T and *mut T
//!         There are C-like raw pointers with no lifetime or ownership attached to them.
//!     # Rc<T>
//!         ## Guarantess
//!             The main guarantee provided here is that the data will not be destroyed until all
//!                 references to it are out of scope.
//!     # Weak<T>
//!         There is a sister smart pointer to this one, Weak<T>. This is a non-owning, but also non-borrowed
//!             smart pointer. It is also similar to &T, but it is not restricted in lifetime -- a Weak<T>
//!             can be held on to forever. However,it is possible that an attempt to accept the inner
//!             data may fail and return None, since this can outlive the owned Rcs. This is useful for
//!             cyclic data structures and other things.
//!     # Cost
//!     # Cell types
//!         Cells provide interior mutability. In other words, they contain data which can be manipulated
//!         even if the type cannot be obtained in a mutable in a mutable from
//!         ## Cell<T>
//!             Cell<T>is a type that provides zero-cost interior mutability by moving data in and out of
//!                 the cell. Since the compiler knows that all the data owned by the contained value is on
//!                 the stack, there's no worry of leaking any data behid references by simply replacing the data.
//!         ## Cost
//!             There is no runtime cost to using Cell<T>, however if you are using it to wrap large
//!                 structs, it might be worthwhile to instead wrap individual fields in Cell<T> since
//!                 each write is otherwise a full copy of the struct.
//!
//!         ## RefCell<T>
//!             RefCell<T> also provides interior mutability, but doesn't move data in and out of the cell.
//!
//!             However, it has a runtime cost. RefCell<T> enforces the read-write lock pattern at runtime(it's like
//!                 a single-threaded mutex), unlike &T/&mut T which do so at compile time. This is done by the
//!                 borrow() and borrow_mut() functions, which modify an internal reference count and return smart
//!                 pointers which can be dereferenced immutably and mutably respectively. The refcount is restored
//!                 when the smart pointers go out of scope. With this system, we can dynamically ensure that there
//!                 are never any other borrows active when a mutable borrow is active. If the programmer attempts
//!                 to make such a borrow, the thread will panic.
//!         ## Cost
//!
//!     # Synchronous types
//!         ## UnsafeCell<T>
//!             UnsafeCell<T> is cell type that can be used to hold any data and has no runtime cost but accessing it
//!                 requires unsafe blocks.
//!             Arc<UnsafeCell<T>> actually wont' compile since UnsafeCell<T> isn't Send or Sync, but we can wrap ti in
//!                 type and implement Send/Sync for it manually to get Arc<Wrapper<T>> where Wrapper is Struct<T>(UnsafeCell<T>)
//!
//!         ## Arc<T>
//!         ## Cost
//!
//!         ## Mutex<T> and RwLock<T>
//!             Mutex<T> and RwLock<T> provide mutual-exclusion via RALL guards (guards are objects which maintain some state, like a
//!                 lock, until their destructor is called). For both of these, the mutex is opaque until we call lock() on it, at which
//!                 point the thread will block can be acquired, and then a guard will be returned. This guard canbe used to access
//!                 the inner data(mutably), and the lock will be released when the guard goes out of scope.
//!
//!             RwLock has the added benefit of being efficient for multiple reads. It is always safe to have multiple
//!                 readers to shared data as long as there are no writers; and RwLock lets readers acquire a "read lock".
//!                 Such locks can be acquired concurrently and are kept track of via a reference count. Writers must obtain a
//!                 "write lock" which can be obtained when all reader have gone out of scope.
//!
//!         ## Cost
//!
//!     # Composition
//!         Rc<RefCell<T>> is one such composition. Rc<T> itself can't be dereferenced mutably; because Re<T> provides inside to
//!             get dynamically verified shared mutability. Now we have shared mutabledata, but it's shared a way that there can
//!             only be one mutator(and no readers) or multiple readers.
//!
//!         Now, we can take this a step further, and have Rc<RefCell<Vec<T>>> or Rec<Vec<RefCell<T>>>.
//!         There are both shareable, mutable vectors, but they're not the same.
//!
//!         With the former, the RefCell<T> is wrapping the Vec<T>, so the Vec<T> in its entirety is mutable.
//!             At the same time, there can only be one mutable borrow of the whole Vec at a given time.
//!             This means that your code cannot simultaneously work on different elements of the vector from
//!             different Rc handles. However, we are able to push and op from the Vec<T> at will. This is similar
//!             to a &mut Vec<T> with the borrow checking done at runtime.
//!
//!         With the latter, the borrowing is of individual elements,but the overall vector is immutable. Thus,
//!             we can independently borrow separate elements, but we cannot push or pop from the vector. This
//!             si similar to a &mut [T], but, again, the borrow checking is at runtime.
//!


#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::cell::RefCell;
    use std::sync::{Mutex, Arc};
    use std::borrow::BorrowMut;

    #[test]
    fn test_006() {
        let mutex = Mutex::new(3);
        let mutex = Arc::new(mutex);
        {
            *mutex.lock().unwrap() += 1;
            // `guard` dereferences mutably to the inner type
            // *guard += 1;
            //
            // *mutex.lock().unwrap() += 1;

        }   // Lock is released when destructor runs.

        println!("{:?}", mutex);
    }

    #[test]
    fn test_005() {
        let x = RefCell::new(vec![1, 2, 3, 4]);
        {
            println!("{:?}", *x.borrow());
        }

        {
            let mut my_ref = x.borrow_mut();
            my_ref.push(1);
        }

        println!("{:?}", x);

    }

    #[test]
    fn test_004() {
        let x = Cell::new(3);
        let y = &x;
        let z = &x;

        x.set(3);
        y.set(3);
        z.set(3);
        println!("{}", x.get());

        // runtime as follow

        //
        // let mut x = 1;
        // let y = &mut x;
        // let z = &mut x;
        // x = 3;
        // *y = 4;
        // *z = 4;
        // println!("{}", x);
    }

    #[test]
    fn test_003() {
        let mut r = 10;

        let p = &mut r as *mut i32;

        unsafe {
            *p = 20;
        }

        println!("{}", r);
    }

    #[test]
    fn test_002() {
        let mut r = 3;
        let pr = &mut r;
        *pr = 44;

        let prr = &r;
        println!("{}", prr);


    }

    #[test]
    fn test_001() {

        let x = Box::new(1);
        let y = x;
        // `x` is no longer accessible here.

    }
}