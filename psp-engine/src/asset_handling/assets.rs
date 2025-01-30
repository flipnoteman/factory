use alloc::alloc::{alloc, dealloc};
use alloc::ffi::CString;
use alloc::format;
use alloc::string::String;
use core::alloc::Layout;
use core::ffi::c_void;
use asset_macros::AssetType;
use core::fmt::Debug;
use core::ptr::slice_from_raw_parts_mut;
use psp::dprintln;
use psp::sys::{sceIoClose, sceIoGetstat, sceIoOpen, sceIoRead, IoOpenFlags, SceIoStat, SceUid};
use crate::utils::*;

pub trait Asset: Debug + AsAny {
    // Doesn't need to be here if we're never using "new()" within a context that may be type agnostic
    // unsafe fn new(filepath: &str) -> Result<Box<dyn Asset>, &str>;
    fn init(&mut self, filepath: String) -> Result<(), &str>;

    fn load(&mut self) -> Result<(), &str>;

}

#[AssetType]
#[derive(Debug, Clone, Copy)]
pub struct Raw;

#[AssetType]
#[derive(Debug, Clone, Copy)]
pub struct BMP {
    header_size: u32,
    width: u32,
    height: u32,
    color_planes: u16,
    bits_per_pixel: u16,
    compression: u32,
    x_pixels_per_meter: u32,
    y_pixels_per_meter: u32,
    colors_used: u32,
    important_colors: u32
}

impl Asset for Raw {
    fn init(&mut self, filepath: String) -> Result<(), &str> {
        unsafe {
            let fd = open_file(filepath.clone(), IoOpenFlags::RD_ONLY)?;

            let stat_layout = Layout::new::<SceIoStat>();
            let stat_handle = alloc(stat_layout) as *mut SceIoStat;

            let c_str = CString::new(filepath.as_str()).unwrap();
            if sceIoGetstat(c_str.as_ptr() as *const u8, stat_handle) < 0 {
                return Err("Failed to get file status.");
            };

            let filesize = (*stat_handle).st_size as u32;

            let layout = Layout::array::<u8>(filesize as usize).unwrap();
            let handle = alloc(layout) as *mut c_void;

            self.handle = Some(handle);
            self.size = filesize;
            self.file_descriptor = fd;
        }

        Ok(())
    }

    fn load(&mut self) -> Result<(), &str> {
        unsafe {

            let handle = self.handle.ok_or("Error, handle not present")?;

            if sceIoRead(self.file_descriptor, handle, self.size) < 0 {
                dealloc(handle as *mut u8, Layout::array::<u8>(self.size as usize).unwrap());
                return Err("Failed to read from file.");
            }

            if sceIoClose(self.file_descriptor) < 0 {
                dealloc(handle as *mut u8, Layout::array::<u8>(self.size as usize).unwrap());
                return Err("Failed to close file.");
            };
        }

        Ok(())
    }
}

impl Asset for BMP {
    fn init(&mut self, filepath: String) -> Result<(), &str> {
        unsafe {
            let fd = open_file(filepath.clone(), IoOpenFlags::RD_ONLY)?;

            let stat_layout = Layout::new::<SceIoStat>();
            let stat_handle = alloc(stat_layout) as *mut SceIoStat;

            let c_str = CString::new(filepath.as_str()).unwrap();
            if sceIoGetstat(c_str.as_ptr() as *const u8, stat_handle) < 0 {
                return Err("Failed to get file status.");
            };

            let filesize = (*stat_handle).st_size as u32;

            let layout = Layout::array::<u8>(filesize as usize).unwrap();
            let handle = alloc(layout) as *mut c_void;

            self.handle = Some(handle);
            self.size = filesize;
            self.file_descriptor = fd;
        }

        Ok(())
    }
    fn load(&mut self) -> Result<(), &str> {
        unsafe {
            let handle = self.handle.ok_or("Error, handle not present")?;

            if sceIoRead(self.file_descriptor, handle, self.size) < 0 {
                dealloc(handle as *mut u8, Layout::array::<u8>(self.size as usize).unwrap());
                return Err("Failed to read from file.");
            }

            if sceIoClose(self.file_descriptor) < 0 {
                dealloc(handle as *mut u8, Layout::array::<u8>(self.size as usize).unwrap());
                return Err("Failed to close file.");
            };

            let size = self.size as usize;

            let h = &*slice_from_raw_parts_mut(handle as *mut u8, self.size as usize);

            let magic: [u8; 2] = *h[0..2].as_array::<2>().unwrap();
            dprintln!("magic: {:?}", magic);
            let filesize = u32::from_le_bytes(*h[2..6].as_array::<4>().unwrap());
            dprintln!("size: {:?}", filesize);
            let offset: u32 = u32::from_le_bytes(*h[10..14].as_array::<4>().unwrap());
            dprintln!("offset: {:?}", offset);

            //TODO: FIX ME
        }
        Ok(())
    }
}