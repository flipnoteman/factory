#![no_std]

extern crate alloc;

use core::{alloc::Layout, ptr::NonNull};
use alloc::alloc::{alloc, dealloc};


#[derive(Debug, Clone)]
pub struct AVec<T> {
    length: usize,
    capacity: usize,
    pointer: NonNull<T>,
}

pub fn realloc(pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {

    let new_layout = Layout::from_size_align(new_size, 16).expect("Couldn't allocate new memory for realloc") ;
    let t = unsafe { alloc(new_layout) };
    let tmp = pointer;

    unsafe {
        core::ptr::copy_nonoverlapping(tmp, t, layout.size());
        dealloc(tmp, layout);
    }

    t 
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
            unsafe { 
                let old_layout = Layout::from_size_align(current_size, 16).expect("Couldn't use layout in realloc");
                let new_size = current_size + (size_of::<T>() * 4);
                self.pointer = NonNull::new(realloc(self.pointer.as_ptr() as *mut u8, old_layout, new_size) as *mut T).expect("Could not create NonNull from realloc");
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

impl<T> core::ops::Deref for AVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe {
            alloc::slice::from_raw_parts(self.pointer.as_ptr(), self.length)
        }
    }
}

impl<T> core::ops::DerefMut for AVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            alloc::slice::from_raw_parts_mut(self.pointer.as_ptr(), self.length)
        }
    }
}

impl<T> Drop for AVec<T> {
    fn drop(&mut self) {
        if self.capacity != 0 {
            
            let layout = Layout::from_size_align(size_of::<T>() * self.capacity, 16).expect("Couldn't create layout in Drop function");
            unsafe {
                dealloc(self.pointer.as_ptr() as *mut u8, layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn capacity_test() {
        let mut t = AVec::new();

        for i in 0..200 {
            t.push(i);
        }

        assert_eq!(t.capacity(), 200);
    }
    
    #[test]
    fn len_test() {
        let mut t = AVec::new();

        for i in 0..200 {
            t.push(i);
        }
        
        assert_eq!(t.len(), 200);
    }
}

