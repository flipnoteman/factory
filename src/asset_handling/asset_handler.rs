use alloc::collections::BTreeMap;
use psp::sys::sceRtcGetCurrentTick;
use alloc::boxed::Box;
use crate::asset_handling::assets::Asset;
use crate::utils::generate_random_number;

pub type Uid = u32;

pub struct AssetHandler {
    pub assets: BTreeMap<Uid, Box::<dyn Asset>>
}

impl AssetHandler {
    pub fn new() -> AssetHandler {
        AssetHandler {
            assets: BTreeMap::new(),
        }
    }

    pub unsafe fn add(&mut self, asset: Box<dyn Asset>) -> Result<Uid, &str> {
        let mut seed: u64 = 0;
        if sceRtcGetCurrentTick(*seed) < 0 {
            return Err("Failed to get current time. Cannot generate random number.");
        }

        let mut uid = generate_random_number(seed);
        while self.assets.try_insert(uid, asset).is_err() {
            uid += 1;
        }

        Ok(uid)
    }
}

