use alloc::ffi::CString;
use core::any::Any;
use psp::sys::{sceIoOpen, IoOpenFlags, SceUid};
use rand::rngs::SmallRng;
use alloc::string::String;
use rand::{RngCore, SeedableRng};

#[inline]
pub fn generate_random_number(seed: u64) -> u32 {
    let mut random = SmallRng::seed_from_u64(seed);
    random.next_u32()
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> AsAny for T
where
    T: 'static,
{
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

pub fn open_file(filepath: String, io_flags: IoOpenFlags) -> Result<SceUid, &'static str> {
    unsafe {
        let path = CString::new(filepath).expect("Error in converting filepath to CString");
        let fd = sceIoOpen(path.as_ptr() as *const u8, io_flags, 0777);
        if fd.0 < 0 { return Err("Failed to open file.") }

        Ok(fd)
    }
}

#[inline]
pub fn is_pow_two(n: u32) -> bool {
    return (n & (n-1) == 0) && n != 0;
}

#[inline]
fn to_ptwo(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut power = 1;
    while power < n {
        power <<= 1;
    }

    if power > 512 {
        return 512
    }
    power
    
}

pub fn convert_ptwo(x: u32, y: u32) -> (u32, u32) {
    match (is_pow_two(x), is_pow_two(y)) {
        (true, true) => (x, y),
        (true, false) => (x, to_ptwo(y)),
        (false, true) => (to_ptwo(x), y),
        _ => (to_ptwo(x), to_ptwo(y)),
   }
}

