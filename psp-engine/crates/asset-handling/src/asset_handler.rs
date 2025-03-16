use alloc::collections::BTreeMap;
use psp::sys::sceRtcGetCurrentTick;
use alloc::boxed::Box;
use core::cell::{RefCell, RefMut, Ref};
use alloc::string::ToString;
use crate::assets::Asset;
use misc::utils::generate_random_number;
use psp_alloc::abump::ABump;

pub type Uid = u32;

struct AssetRef {
    file_path: alloc::string::String,
    uuid: Uid,
    asset: RefCell::<Box::<dyn Asset>>
}

pub struct AssetHandler {
    length: usize,
    assets: BTreeMap<Uid, AssetRef>,
    assets_data: RefCell<ABump>,
}

impl AssetHandler {
    pub fn new() -> AssetHandler {
        AssetHandler {
            length: 0,
            assets: BTreeMap::new(),
            assets_data: RefCell::new(ABump::new())
        }
    }

    pub fn len(&self) -> usize { self.length }

    pub fn add<T>(&mut self, filepath: &str) -> Result<Uid, &str>
    where
        T: Asset + Clone + Default + 'static,
    {
        let mut seed: u64 = 0;
        if unsafe { sceRtcGetCurrentTick(&mut seed as *mut u64) } < 0 {
            return Err("Failed to get current time. Cannot generate random number.");
        }

        let uid = generate_random_number(seed);
        
        let mut asset = T::default();
        if asset.init(filepath.to_string()).is_err() {
            return Err("Failed to init asset.");
        }

        let aref = AssetRef {
            file_path: filepath.to_string(),
            uuid: uid,
            asset: RefCell::new(Box::new(asset)),
        };

        self.assets.insert(uid, aref);

        Ok(uid)
    }

    pub fn load(&self, uid: Uid) -> Result<(), &str> {
        match self.assets.get(&uid).expect("Could not find asset").asset.borrow_mut().load(uid, self.assets_data.borrow_mut()) {
            Ok(_) => Ok(()),
            Err(_) => Err("Error in loading data from asset")
        }
    }

    pub fn query<T>(&self, uid: Uid) -> Result<Ref<T>, &str>
    where
        T: Asset + 'static,
    {
        match self.assets.get(&uid) {
            None => Err("Query failed to find asset."),
            Some(x) => Ok(Ref::map(x.asset.borrow(), |x| x.as_any().downcast_ref::<T>().unwrap()))
        }
    }

    pub fn query_mut<T>(&self, uid: Uid) -> Result<RefMut<T>, &str>
    where
        T: Asset + 'static,
    {
        match self.assets.get(&uid) {
            None => Err("Query failed to find asset"),
            Some(x) => Ok(RefMut::map(x.asset.borrow_mut(), |x| x.as_any_mut().downcast_mut::<T>().unwrap()))
        }
    }
}
