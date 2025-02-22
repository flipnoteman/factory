#![no_std]

extern crate alloc;

use core::{alloc::Layout, ptr::NonNull};
use alloc::alloc::{alloc, dealloc};


#[derive(Debug, Clone, Copy)]
pub struct AVec<T> {
    length: usize,
    capacity: usize,
    pointer: NonNull<T>,
}

impl<T> AVec<T> {
    /// Creates a new [`AVec<T>`].
    pub fn new() -> AVec<T> {
        AVec {
            length: 0,
            capacity: 0,
            pointer: NonNull::dangling(),
        }
    }

    pub fn push(&mut self, value: T) {
        assert_ne!(size_of::<T>(), 0, "Zero-sized types not allowed.");
        if self.capacity == 0 {
            let layout = Layout::from_size_align(size_of::<T>() * 4, 16).expect("Could not allocate with layout");
            let t = unsafe { alloc(layout) } as *mut T;
            let t = NonNull::new(t).expect("Couldn't allocate memory");
            unsafe { t.as_ptr().write(value) };
            self.pointer = t;
            self.capacity = 4;
            self.length = 1;
        } else if self.length < self.capacity {
            unsafe { self.pointer.as_ptr().add(self.length).write(value) };
            self.length += 1;
        } else {
            let current_size = self.capacity * size_of::<T>();
            let new_layout = Layout::from_size_align(current_size + (size_of::<T>() * 4), 16).expect("Couldn't allocate new memory for realloc") ;
            let t = unsafe { alloc(new_layout) } as *mut T;
            let t = NonNull::new(t).expect("Couldn't allocate new memory");
            let tmp = self.pointer;
            self.pointer = t;
            unsafe { 
                core::ptr::copy_nonoverlapping(tmp.as_ptr(), t.as_ptr(), self.length);
                dealloc(tmp.as_ptr() as *mut u8, Layout::from_size_align(current_size, 16).expect("Couldn't deallocate memory for realloc"));
                self.pointer.as_ptr().add(self.length).write(value);
            };
            self.length += 1;
            self.capacity += 4;
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn capacity_test() {
        let mut t = AVec::new();

        t.push(1u8); 
        t.push(2u8);
        t.push(3u8);
        t.push(4u8);
        t.push(5u8); // Should reallocate memory on 5th call
        
        assert_eq!(t.capacity(), 8);
    }
}

