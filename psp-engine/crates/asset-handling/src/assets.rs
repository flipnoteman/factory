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
use misc::utils::*;

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
    pub bits: u16,
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
            // Think about how this operates, if something goes wrong, do we want the file to still
            // be open?
            let fd = open_file(filepath.clone(), IoOpenFlags::RD_ONLY)?;

            let stat_layout = Layout::new::<SceIoStat>();
            let stat_handle = alloc(stat_layout) as *mut SceIoStat;

            let c_str = CString::new(filepath.as_str()).unwrap();
            if sceIoGetstat(c_str.as_ptr() as *const u8, stat_handle) < 0 {
                dealloc(stat_handle as *mut u8, stat_layout);
                return Err("Failed to get file status.");
            };

            let filesize = (*stat_handle).st_size as u32;

            self.size = filesize;
            self.file_descriptor = fd;
        }

        Ok(())
    }

    fn load(&mut self) -> Result<(), &str> {
        unsafe {

            let layout = Layout::from_size_align(self.size as usize, 16).unwrap();
            let handle = alloc(layout) as *mut c_void;
            
            let dealloc_layout = Layout::from_size_align(self.size as usize, 16).unwrap();
            if sceIoRead(self.file_descriptor, handle, self.size) < 0 {
                dealloc(handle as *mut u8, dealloc_layout);
                return Err("Failed to read from file.");
            }

            if sceIoClose(self.file_descriptor) < 0 {
                dealloc(handle as *mut u8, dealloc_layout);
                return Err("Failed to close file.");
            };
            
            self.handle = Some(handle);
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

            self.size = filesize;
            self.file_descriptor = fd;
        }

        Ok(())
    }
    fn load(&mut self) -> Result<(), &str> {
        unsafe {
            let size = self.size as usize;

            let tmp_layout = Layout::array::<u8>(size).unwrap();
            let tmp_handle = alloc(tmp_layout) as *mut c_void;
            
            // Read in header data to temporary handle
            if sceIoRead(self.file_descriptor, tmp_handle, size as u32) < 0 {
                dealloc(tmp_handle as *mut u8, Layout::array::<u8>(size).unwrap());
                return Err("Failed to read from file.");
            }
            
            // Get data from header
            let h = &*slice_from_raw_parts_mut(tmp_handle as *mut u8, size);
            let header_size: u32 = u32::from_le_bytes(*h[14..18].as_array::<4>().unwrap());

            self.bih.header_size = header_size;
            self.offset = u32::from_le_bytes(*h[0x0A..0x0E].as_array::<4>().unwrap());
            self.bih.bits = u16::from_le_bytes(*h[0x1C..0x1E].as_array::<2>().unwrap());
            self.bih.width = u32::from_le_bytes(*h[18..22].as_array::<4>().unwrap());
            self.bih.height = u32::from_le_bytes(*h[22..26].as_array::<4>().unwrap()); 
            
            let row_size = if self.bih.bits == 32 { self.bih.width * 4 } else { self.bih.width * 3 };
            let pad_s = (4 - (row_size % 4)) % 4; // Number of bytes to pad each row
            let data_size = self.bih.width * self.bih.height * 4; // Now we must account for there being an alpha channel
                        
            // Reallocate the memory so that theres enough room for the transformed data
            self.handle = Some(alloc(Layout::from_size_align(data_size as usize, 16).unwrap()) as *mut c_void);
            let handle = self.handle.ok_or("Error unwrapping handle").unwrap() as *mut u8;
            
            self.size = data_size;
            
            // Close file
            if sceIoClose(self.file_descriptor) < 0 {
                dealloc(handle, Layout::from_size_align(data_size as usize, 16).unwrap());
                dealloc(tmp_handle as *mut u8, Layout::array::<u8>(size).unwrap());
                return Err("Failed to close file.");
            };

            let src_ptr = (tmp_handle as *mut u8).add(self.offset as usize);

            // Now go through all the given pixels and build raw data 
            for y in 0..self.bih.height {

                // Starting row
                let src_row = if self.bih.height > 0 { self.bih.height - y - 1} else { y };
                let src_start = src_row * (row_size + pad_s); // 

                let dest_start = y * self.bih.width * 4;
 
                for x in 0..self.bih.width {
                    
                    let (src_pixel, dest_pixel) = match self.bih.bits {
                        32 => (src_start + (x * 4), dest_start + (x * 4)),
                        _ => (src_start + (x * 3), dest_start + (x * 4))

                    };

                    let b = *src_ptr.add(src_pixel as usize);
                    let g = *src_ptr.add(src_pixel as usize + 1);
                    let r = *src_ptr.add(src_pixel as usize + 2);

                    handle.add(dest_pixel as usize).write(r);
                    handle.add(dest_pixel as usize + 1).write(g);
                    handle.add(dest_pixel as usize + 2).write(b);
                    
                    
                    if self.bih.bits != 32 {
                        handle.add(dest_pixel as usize + 3).write(0xFF);
                    } else {
                        let a = *src_ptr.add(src_pixel as usize + 3);
                        handle.add(dest_pixel as usize + 3).write(a);
                    }
                }
            }
           
            dealloc(tmp_handle as *mut u8, Layout::array::<u8>(size).unwrap());
         
            //TODO: See if compression is possible and if it can be implemented
        }
        Ok(())
    }
}
