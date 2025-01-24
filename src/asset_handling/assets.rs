use alloc::alloc::alloc;
use alloc::boxed::Box;
use alloc::ffi::CString;
use core::alloc::Layout;
use core::any::Any;
use core::ffi::c_void;
use asset_macros::AssetType;
use core::fmt::Debug;
use core::ptr::null_mut;
use psp::sys::{sceIoClose, sceIoGetstat, sceIoOpen, sceIoRead, IoOpenFlags, SceIoStat, SceUid};

pub trait Asset: Debug {

    // Doesn't need to be here if we're never using "new()" within a context that may be type agnostic
    // unsafe fn new(filepath: &str) -> Result<Box<dyn Asset>, &str>;
    unsafe fn init(&mut self, filepath: &str) -> Result<(), &str>;
    unsafe fn load(&mut self) -> Result<(), &str>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[AssetType]
#[derive(Debug, Clone, Copy)]
pub struct BMP;

#[AssetType]
#[derive(Debug, Clone, Copy)]
pub struct Raw {
    pub size: u32,
}

impl Default for Raw {
    fn default() -> Self {
        Raw {
            size: 0,
            handle: null_mut(),
            file_descriptor: SceUid(1),
        }
    }
}

impl Asset for Raw {
    unsafe fn init(&mut self, filepath: &str) -> Result<(), &str> {
        let path = CString::new(filepath).expect("Error in converting filepath to CString");
        let fd = sceIoOpen(path.as_ptr() as *const u8, IoOpenFlags::RD_ONLY, 0777);
        if fd.0 < 0 { return Err("Failed to open file.") }

        let stat_layout = Layout::new::<SceIoStat>();
        let stat_handle = alloc(stat_layout) as *mut SceIoStat;

        if sceIoGetstat(path.as_ptr() as *const u8, stat_handle) < 0 {
            return Err("Failed to get file status.");
        };

        let filesize = (*stat_handle).st_size as u32;

        let layout = Layout::array::<u8>(filesize as usize).unwrap();
        let handle = alloc(layout) as *mut c_void;

        self.handle = handle;
        self.size = filesize;
        self.file_descriptor = fd;

        Ok(())
    }

    unsafe fn load(&mut self) -> Result<(), &str> {
        if sceIoRead(self.file_descriptor, self.handle, self.size) < 0 {
            return Err("Failed to read Ferris texture");
        }

        if sceIoClose(self.file_descriptor) < 0 {
            return Err("Failed to close file.");
        };

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}