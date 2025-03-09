use crate::avec::AVec;
use core::ptr::NonNull;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone)]
struct Element(NonNull<u8>, usize);

#[derive(Debug, Clone)]
pub struct ABump {
    data: AVec<u8>,
    meta: BTreeMap<alloc::string::String, Element>,
}

impl Element {
    fn size(&self) -> usize {
        self.1
    }

    fn pointer(&self) -> *mut u8 {
        self.0.as_ptr()
    }

}


impl ABump {

    pub fn new() -> ABump {
        ABump {
            meta: BTreeMap::new(),
            data: AVec::new(),
        }
    }

    pub fn append(&mut self, name: alloc::string::String, pointer: *const u8, size: usize) -> Result<usize, alloc::string::String> {

        self.data.reserve_exact(size);
        
//         for i in 0..size {
//             unsafe {
//                 self.data.push(pointer.add(i).read());
//             }
//         }
        
        let slice = unsafe { core::slice::from_raw_parts(pointer, size) };
        self.data.extend_from_slice(slice);
        
        let e = unsafe { Element(NonNull::new(self.data.as_ptr().add(size) as *mut _).unwrap(), size) };
        
        self.meta.insert(name, e);
        
        Ok(self.meta.len())
    }

    pub fn get(&mut self, name: &alloc::string::String) -> Option<(*mut u8, usize)> {
        let r = self.meta.get(name);
        match r {
            Some(e) => Some((e.pointer(), e.size())),
            None => None,
        }
    }
    
    pub fn len(&self) -> usize { self.meta.len() }
    pub fn capacity(&self) -> usize { self.data.capacity() }
    pub fn raw_len(&self) -> usize { self.data.len() }
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_test() {
        let mut t = ABump::new();
        let data = [1u8, 2, 3];

        let result = t.append(alloc::string::String::from("test"), data.as_ptr(), data.len());
        assert!(result.is_ok());
        assert_eq!(t.len(), 1);

        let (_, size) = t.get(&alloc::string::String::from("test")).unwrap();
        assert_eq!(size, 3);
    }

    #[test]
    fn len_test() {}

    #[test]
    fn capacity_test() {}

    #[test]
    fn raw_len_test() {}

    #[test]
    fn get_test() {}
}
