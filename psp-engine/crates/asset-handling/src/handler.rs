use super::assets::Asset;
use misc::utils::*;
use alloc::{collections::BTreeMap, string::ToString, boxed::Box };
use core::cell::{Ref, RefCell, RefMut};
use psp::sys::sceRtcGetCurrentTick;

pub type Uid = u32;

pub struct AssetHandler {
    pub assets: BTreeMap<Uid, RefCell<Box<dyn Asset>>>,
}

impl AssetHandler {
    pub fn new() -> AssetHandler {
        AssetHandler {
            assets: BTreeMap::new(),
        }
    }

    pub fn add<T>(&mut self, filepath: &str) -> Result<Uid, &str>
    where
        T: Asset + Clone + Default + 'static,
    {
        unsafe {
            let mut seed: u64 = 0;
            if sceRtcGetCurrentTick(&mut seed as *mut u64) < 0 {
                return Err("Failed to get current time. Cannot generate random number.");
            }

            let mut asset = T::default();
            if asset.init(filepath.to_string()).is_err() {
                return Err("Failed to init asset.");
            }

            let mut uid = generate_random_number(seed);
            while self
                .assets
                .try_insert(uid, RefCell::new(Box::new(asset.clone())))
                .is_err()
            {
                seed += 1;
                uid = generate_random_number(seed);
            }

            Ok(uid)
        }
    }

    pub fn query<T>(&self, uid: Uid) -> Result<Ref<T>, &str>
    where
        T: Asset + 'static,
    {
        match self.assets.get(&uid) {
            None => Err("Query failed to find asset."),
            Some(x) => Ok(Ref::map(x.borrow(), |x| {
                x.as_any().downcast_ref::<T>().unwrap()
            })),
        }
    }

    pub fn query_mut<T>(&self, uid: Uid) -> Result<RefMut<T>, &str>
    where
        T: Asset + 'static,
    {
        match self.assets.get(&uid) {
            None => Err("Query failed to find asset"),
            Some(x) => Ok(RefMut::map(x.borrow_mut(), |x| {
                x.as_any_mut().downcast_mut::<T>().unwrap()
            })),
        }
    }
}
