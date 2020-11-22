
//!
//!
//! # Destructors
//!

use std::alloc::{AllocRef, Global, GlobalAlloc, Layout};
use std::ptr::{drop_in_place, Unique, NonNull};
use std::mem;

struct Box<T>{ ptr: Unique<T> }

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe {
            drop_in_place(self.ptr.as_ptr());
            let c: NonNull<T> = self.ptr.into();
            Global.dealloc(c.cast(), Layout::new::<T>());
        }
    }
}

struct SuperBox<T> { my_box: Box<T> }

impl<T> Drop for SuperBox<T> {
    fn drop(&mut self) {
        unsafe {
            // Hyper-optimized: deallocate the box's contents for it
            // without `drop`ing the contents
            let c: NonNull<T> = self.my_box.ptr.into();
            Global.dealloc(c.cast::<u8>(), Layout::new::<T>());
        }
    }
}

struct Box2<T>{ ptr: Unique<T> }

impl<T> Drop for Box2<T> {
    fn drop(&mut self) {
        unsafe {
            drop_in_place(self.ptr.as_ptr());
            let c: NonNull<T> = self.ptr.into();
            Global.dealloc(c.cast(), Layout::new::<T>());
        }
    }
}

struct SuperBox2<T> { my_box: Option<Box<T>> }

impl<T> Drop for SuperBox2<T> {
    fn drop(&mut self) {
        unsafe {
            // Hyper-optimized: deallocate the box's contents for it
            // without `drop`ing the contents. Need to set the `box`
            // field as `None` to prevent Rust from trying to Drop it.
            let my_box = self.my_box.take().unwrap();
            let c: NonNull<T> = my_box.ptr.into();
            Global.dealloc(c.cast(), Layout::new::<T>());
            mem::forget(my_box);
        }
    }
}


#[test]
fn test_001() {

}