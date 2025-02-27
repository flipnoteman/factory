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
    let new_layout = Layout::from_size_align(new_size, layout.align()).expect("Couldn't allocate new memory for realloc") ;
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

    pub fn with_capacity(capacity: usize) -> AVec<T> {
        let layout = Layout::from_size_align(size_of::<T>() * capacity, 16).expect("Could not allocate with layout");
        let t = unsafe { alloc(layout) } as *mut T;
        let t = NonNull::new(t).expect("Couldn't allocate memory");

        AVec {
            length: 0,
            capacity,
            pointer: t
        }
    }
    
    pub fn reserve(&mut self, additional: usize) {
        if self.capacity == 0 {
            let size = additional + ((4 - additional % 4) % 4);
            let layout = Layout::from_size_align(size_of::<T>() * size, 16).expect("Could not allocate with layout");
            let t = unsafe { alloc(layout) } as *mut T;
            let t = NonNull::new(t).expect("Couldn't allocate memory");
            self.pointer = t;
            self.capacity = size;
            self.length = 0;
        } else 
        if self.capacity - self.length > additional { 
            return;
        } else {
            let adj_add = additional + ((4 - additional % 4) % 4);
            let new_size = (self.capacity * size_of::<T>()) + adj_add * size_of::<T>();
            self.capacity = self.capacity + adj_add;
            self.pointer = NonNull::new(realloc(self.pointer.as_ptr() as *mut u8, Layout::from_size_align(size_of::<T>() * self.capacity, 16).unwrap(), new_size) as *mut T).unwrap();
        }
    }


    pub fn reserve_exact(&mut self, additional: usize) {
        if self.capacity == 0 {
            let layout = Layout::from_size_align(size_of::<T>() * additional, 16).expect("Could not allocate with layout");
            let t = unsafe { alloc(layout) } as *mut T;
            let t = NonNull::new(t).expect("Couldn't allocate memory");
            self.pointer = t;
            self.capacity = additional;
            self.length = 0;
        } else 
        if self.capacity - self.length > additional { 
            return;
        } else {
            self.capacity = self.capacity + additional;

            let old = self.capacity * size_of::<T>();
            let new = old + (additional * size_of::<T>());

            self.pointer = NonNull::new(realloc(self.pointer.as_ptr() as *mut u8, Layout::from_size_align(size_of::<T>() * self.capacity, 16).unwrap(), new) as *mut T).unwrap();
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }
        unsafe {
            Some(&*self.pointer.as_ptr().add(index))
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

    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            None
        } else {
            self.length -= 1;
            unsafe { Some(self.pointer.as_ptr().add(self.length).read()) }
        }
    }

    pub fn insert(&mut self, index: usize, value: T) {
        if self.length == self.capacity { self.reserve(1) }; 
        assert!(index <= self.length, "index out of bounds");
        
        unsafe {
            self.pointer.as_ptr().add(index + 1).copy_from(self.pointer.as_ptr().add(index), self.length - index);
            self.pointer.as_ptr().add(index).write(value);
            self.length += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.length, "index out of bounds");

        unsafe {
            let result = self.pointer.as_ptr().add(index).read();
            self.length -= 1; 
            self.pointer.as_ptr().add(index).copy_from(self.pointer.as_ptr().add(index + 1), self.length - index);
            result
        }
    }

    pub fn swap_remove(&mut self, index: usize) -> T {
        assert!(index < self.length, "index out of bounds");

        unsafe {
            let result = self.pointer.as_ptr().add(index).read();
            self.length -= 1; 
            self.pointer.as_ptr().add(index).copy_from(self.pointer.as_ptr().add(self.length), 1);
            result
        }
    }

//     pub fn append(&mut self, other: &mut AVec<T>) {
//         for i in 0..other.len() {
// 
//             self.push(*other.get(i).unwrap())
// 
//         }
//     }
    
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

unsafe impl<T: Send> Send for AVec<T> {}
unsafe impl<T: Sync> Sync for AVec<T> {}

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
                core::ptr::drop_in_place(alloc::slice::from_raw_parts_mut(self.pointer.as_ptr(), self.length));
                dealloc(self.pointer.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T> PartialEq for AVec<T>
where T: PartialEq
{
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &Self) -> bool {
        let mut r = match self.len() == other.len() {
            true => true,
            false => { return false }
        };
        
        for i in 0..self.length {
            if *self.get(i).unwrap() != *other.get(i).unwrap() {
               r = false; 
            }
        }
        r
    }
}

impl<T: Copy> From<&[T]> for AVec<T> {
    fn from(value: &[T]) -> Self {
        let mut r: AVec<T> = AVec::new();
        
        for i in 0..value.len() {
            r.push(value[i])
        }

        r
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

    #[test]
    fn with_capacity_test() {
        let t: AVec<u8> = AVec::with_capacity(60);

        assert_eq!(t.capacity(), 60);
        assert_eq!(t.len(), 0);
    }
    
    #[test]
    fn get_test() {
        let mut t = AVec::new();

        t.push(1); 
        t.push(2); 
        t.push(3); 
        t.push(4); 
        t.push(5); 

        for n in 0..t.len() {
            assert_eq!(t.get(n), Some(&(n + 1)));
        }

        assert_eq!(t.capacity(), 8);
        assert_eq!(t.len(), 5);
    }
    
    #[test]
    fn pop_test() {
        let mut t = AVec::new();

        t.push(1); 
        t.push(2); 
        t.push(3); 
        t.push(4); 
        t.push(5); 

        let r = t.pop();

        assert_eq!(t.len(), 4); 
        assert_eq!(t.capacity(), 8); 
        assert_eq!(Some(5), r); 
    }
    
    #[test]
    fn reserve_exact_test() {
        let mut t: AVec<u8> = AVec::new();

        assert_eq!(t.capacity(), 0);

        t.reserve_exact(15);

        assert_eq!(t.capacity(), 15);

    }

    #[test]
    fn reserve_test() {
        let mut t: AVec<u8> = AVec::new();

        assert_eq!(t.capacity(), 0);

        t.reserve(15);

        assert_eq!(t.capacity(), 16);

    }

    #[test]
    fn insert_test() {
        let mut t: AVec<u8> = AVec::new();

        t.push(2);
        t.insert(0, 1);

        let mut r: AVec<u8> = AVec::new();

        r.push(1);
        r.insert(1, 2);

        assert_eq!(t, r);
    }

    #[test]
    fn remove_test() {
        let mut t: AVec<u8> = AVec::new();
        
        t.push(1); 
        t.push(2); 
        t.push(3); 
        t.push(4); 

        assert_eq!(t.remove(1), 2);
        
        let mut r: AVec<u8> = AVec::new();
        
        r.push(1);
        r.push(3);
        r.push(4);
        
        assert_eq!(t, r);
    }

    #[test]
    fn swap_remove_test() {
        let mut t: AVec<u8> = AVec::new();
        
        t.push(1); 
        t.push(2); 
        t.push(3); 
        t.push(4); 

        assert_eq!(t.swap_remove(1), 2);
        let mut r: AVec<u8> = AVec::new();
        
        r.push(1);
        r.push(4);
        r.push(3);
        
        assert_eq!(t, r);

    }
}

