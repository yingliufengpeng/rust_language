//!
//! # Exception Safety
//!
//!
//!
//!

use std::ptr;
use std::collections::BinaryHeap;

struct Hole<'a, T: 'a> {
    data: &'a mut [T],
    /// `elt` is always `Some` from new until drop.
    elt: Option<T>,
    pos: usize,
}

impl<'a, T> Hole<'a, T> {
    fn new(data: &'a mut [T], pos: usize) -> Self {
        unsafe {
            let elt = ptr::read(&data[pos]);
            Hole {
                data: data,
                elt: Some(elt),
                pos: pos,
            }
        }
    }

    fn pos(&self) -> usize { self.pos }

    fn removed(&self) -> &T { self.elt.as_ref().unwrap() }

    unsafe fn get(&self, index: usize) -> &T { &self.data[index] }

    unsafe fn move_to(&mut self, index: usize) {
        let index_ptr: *const _ = &self.data[index];
        let hole_ptr = &mut self.data[self.pos];
        ptr::copy_nonoverlapping(index_ptr, hole_ptr, 1);
        self.pos = index;
    }
}

impl<'a, T> Drop for Hole<'a, T> {
    fn drop(&mut self) {
        // fill the hole again
        unsafe {
            let pos = self.pos;
            ptr::write(&mut self.data[pos], self.elt.take().unwrap());
        }
    }
}

// impl<T: Ord> BinaryHeap<T> {
//     fn sift_up(&mut self, pos: usize) {
//         // unsafe {
//         //     // Take out the value at `pos` and create a hole.
//         //     let mut hole = Hole::new(&mut self.data, pos);
//         //
//         //     while hole.pos() != 0 {
//         //         let parent = parent(hole.pos());
//         //         if hole.removed() <= hole.get(parent) { break }
//         //         hole.move_to(parent);
//         //     }
//         //     // Hole will be unconditionally filled here; panic or not!
//         // }
//     }
// }

#[cfg(test)]
mod tests {

    #[test]
    fn test_001() {


    }

}