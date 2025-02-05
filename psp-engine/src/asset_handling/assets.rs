use alloc::alloc::{alloc, dealloc, realloc};
use alloc::ffi::CString;
use alloc::string::String;
use psp::dprintln;
use core::alloc::Layout;
use core::ffi::c_void;
use asset_macros::AssetType;
use core::fmt::Debug;
use core::ptr::{null_mut, slice_from_raw_parts_mut};
use psp::sys::{sceIoClose, sceIoGetstat, sceIoRead, IoOpenFlags, SceIoStat};
use crate::utils::*;

pub trait Asset: Debug + AsAny {
    fn init(&mut self, filepath: String) -> Result<(), &str>;

    fn load(&mut self) -> Result<(), &str>;
}

#[AssetType]
#[derive(Debug, Clone, Copy)]
pub struct Raw;

#[derive(Default, Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct BIH {
    pub header_size: u32,
    pub width: u32,
    pub height: u32,
//  pub    color_planes: u16,
//  pub    bits_per_pixel: u16,
    pub compression: u32,
//     image_size:u32,
//     x_pixels_per_meter: u32,
//     y_pixels_per_meter: u32,
//     colors_used: u32,
//     important_colors: u32
}


#[AssetType]
#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct BMP {
    pub offset: u32,
    pub bih: BIH,
}

impl Asset for Raw {
    fn init(&mut self, filepath: String) -> Result<(), &str> {
        unsafe {
            let fd = open_file(filepath.clone(), IoOpenFlags::RD_ONLY)?;
            

            let stat_layout = Layout::new::<SceIoStat>();
            let stat_handle = alloc(stat_layout) as *mut SceIoStat;

            let c_str = CString::new(filepath.as_str()).unwrap();
            if sceIoGetstat(c_str.as_ptr() as *const u8, stat_handle) < 0 {
                dealloc(stat_handle as *mut u8, stat_layout);
                return Err("Failed to get file status.");
            };

            let filesize = (*stat_handle).st_size as u32;

            let layout = Layout::from_size_align(filesize as usize, 16).unwrap();
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
            
            let dealloc_layout = Layout::from_size_align(self.size as usize, 16).unwrap();
            if sceIoRead(self.file_descriptor, handle, self.size) < 0 {
                dealloc(handle as *mut u8, dealloc_layout);
                return Err("Failed to read from file.");
            }

            if sceIoClose(self.file_descriptor) < 0 {
                dealloc(handle as *mut u8, dealloc_layout);
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
                dealloc(stat_handle as *mut u8, stat_layout);
                return Err("Failed to get file status.");
            };


            let filesize = (*stat_handle).st_size as u32;

            let layout = Layout::from_size_align(0x0 as usize, 16).unwrap();
            let handle = alloc(layout) as *mut c_void;

            self.handle = Some(handle);
            self.size = filesize;
            self.file_descriptor = fd;
        }

        Ok(())
    }
    fn load(&mut self) -> Result<(), &str> {
        unsafe {
            let size = self.size as usize;
            let handle = self.handle.ok_or("Error, handle not present")? as *mut u8;

            let tmp_layout = Layout::from_size_align(0x40, 16).unwrap();
            let tmp_handle = alloc(tmp_layout) as *mut c_void;
            
            // Read in header data to temporary handle
            if sceIoRead(self.file_descriptor, tmp_handle, 0x40) < 0 {
                dealloc(tmp_handle as *mut u8, Layout::from_size_align(size, 16).unwrap());
                return Err("Failed to read from file.");
            }
            
            // Get data from header
            let h = &*slice_from_raw_parts_mut(tmp_handle as *mut u8, size);

            // Check magic to make sure file is valid, THIS PART MAY BE UNNECCESARY AND JUST SLOW
            // THINGS DOWN
            let magic: [u8; 2] = *h[0..2].as_array::<2>().unwrap();
            if magic[0] != 0x42 || magic[1] != 0x4D {
                dealloc(tmp_handle as *mut u8, Layout::from_size_align(size, 16).unwrap());
                return Err("File magic does not match BMP format.");
            }
            
            let header_size: u32 = u32::from_le_bytes(*h[14..18].as_array::<4>().unwrap());
            if header_size > 40 {
                dealloc(tmp_handle as *mut u8, Layout::from_size_align(size, 16).unwrap());
                return Err("Unsupported BMP type, header is larger than 40 bytes");
            }

            self.bih.header_size = header_size;
            self.offset = u32::from_le_bytes(*h[10..14].as_array::<4>().unwrap());
            self.bih.width = u32::from_le_bytes(*h[18..22].as_array::<4>().unwrap());
            self.bih.height = u32::from_le_bytes(*h[22..26].as_array::<4>().unwrap()); 
            let image_size = u32::from_le_bytes(*h[0x22..0x26].as_array::<4>().unwrap()); 
            
            let pad_s = 4 - (self.bih.width % 4); // Number of bytes to pad each row
            let pad_n = pad_s * self.bih.height; // Total number of pad bytes
            let pixel_n = (image_size - pad_n) / 3; // Get pixel count, should always be multiple of 3

            let data_size = pixel_n * 4; // Now we must account for there being an alpha channel
            
            dprintln!("{:?}", pixel_n);
            // Reallocate the memory so that theres enough room for the transformed data
            let _ = realloc(handle, Layout::from_size_align(0x00, 16).unwrap(), data_size as usize);
            self.size = data_size;
            
            if sceIoClose(self.file_descriptor) < 0 {
                dealloc(handle, Layout::from_size_align(data_size as usize, 16).unwrap());
                dealloc(tmp_handle as *mut u8, Layout::from_size_align(size, 16).unwrap());
                return Err("Failed to close file.");
            };

            
            for i in 0..pixel_n {
                let index = 0x4 * i as usize;
                handle.add(index + 3).write(0xFF);
                handle.add(index).copy_from((tmp_handle as *mut u8).add(self.offset as usize + index), 3);
            }
           
            //TODO: See if compression is possible and if it can be implemented
         
        }
        Ok(())
    }
}
