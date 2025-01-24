use alloc::collections::BTreeMap;
use psp::sys::sceRtcGetCurrentTick;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::ToString;
use crate::asset_handling::assets::{Asset, Raw};
use crate::utils::generate_random_number;

pub type Uid = u32;

pub struct AssetHandler {
    pub assets: BTreeMap<Uid, Box<dyn Asset>>
}

impl AssetHandler {
    pub fn new() -> AssetHandler {
        AssetHandler {
            assets: BTreeMap::new(),
        }
    }

    pub unsafe fn add<T>(&mut self, filepath: &str) -> Result<Uid, &str>
    where
        T: Asset + Clone + Default + 'static,
    {
        let seed: u64 = 0;
        if sceRtcGetCurrentTick(seed as *mut u64 ) < 0 {
            return Err("Failed to get current time. Cannot generate random number.");
        }

        let mut asset = T::default();
        if asset.init(filepath).is_err() {
            return Err("Failed to init asset.");
        }

        let mut uid = generate_random_number(seed);
        while self.assets.try_insert(uid, Box::new(asset.clone())).is_err() {
            uid += 1;
        }

        Ok(uid)
    }

    pub fn query<T>(&self, uid: Uid) -> Result<&T, &str>
    where
        T: Asset + 'static,
    {
        match self.assets.get(&uid) {
            None => Err("Query failed to find asset."),
            Some(x) => Ok(x.as_any().downcast_ref::<T>().unwrap())
        }
    }

    pub fn query_mut<T>(&mut self, uid: Uid) -> Result<&mut T, &str>
    where
        T: Asset + 'static,
    {
        match self.assets.get_mut(&uid) {
            None => Err("Query failed to find asset"),
            Some(x) => Ok(x.as_any_mut().downcast_mut::<T>().unwrap())
        }
    }
}