use alloc::alloc::alloc;
use alloc::collections::BTreeMap;
use alloc::ffi::CString;
use core::ffi::c_void;
use core::alloc::Layout;
use psp::sys::{sceIoClose, sceIoGetstat, sceIoOpen, sceIoRead, IoOpenFlags, SceIoStat, SceUid};
use crate::utils::generate_random_number;

pub type Handle = u32;

pub struct AssetHandler {
    pub assets: BTreeMap<Handle, AssetHandle>
}

#[derive(Debug, Clone, Copy)]
pub struct AssetHandle {
    pub handle: *mut c_void,
    file_descriptor: SceUid,
    pub size: u32,
}

impl AssetHandler {
    pub fn new() -> AssetHandler {
        AssetHandler {
            assets: BTreeMap::new(),
        }
    }

    pub unsafe fn add(&mut self, filepath: &str) -> Result<Handle, &str> {

        let path = CString::new(filepath).expect("Error in converting filepath to CString");
        let fd = sceIoOpen(path.as_ptr() as *const u8, IoOpenFlags::RD_ONLY, 0777);
        if fd.0 < 0 { return Err("Failed to open file.") }

        let stat_layout = Layout::new::<SceIoStat>();
        let stat_handle = alloc(stat_layout) as *mut SceIoStat;

        if sceIoGetstat(path.as_ptr() as *const u8, stat_handle) < 0 {
            return Err("Failed to get file status.");
        };

        let filesize = (*stat_handle).st_size as u32;
        let seed = (*stat_handle).st_atime.seconds as u64;

        let layout = Layout::array::<u8>(filesize as usize).unwrap();
        let handle = alloc(layout) as *mut c_void;

        let asset = AssetHandle {
            handle,
            file_descriptor: fd,
            size: filesize,
        };

        let mut rand_num = generate_random_number(seed);

        while self.assets.try_insert(rand_num, asset).is_err() {
            rand_num += 1;
        }

        Ok(rand_num)
    }
}

impl AssetHandle {
    pub unsafe fn load(&mut self) -> Result<(), &str> {
        if sceIoRead(self.file_descriptor, self.handle, self.size) < 0 {
            return Err("Failed to read Ferris texture");
        }

        if sceIoClose(self.file_descriptor) < 0 {
            return Err("Failed to close file.");
        };

        Ok(())
    }
}