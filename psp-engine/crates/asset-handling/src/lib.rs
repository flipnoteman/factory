#![no_std]
#![feature(map_try_insert)]
#![feature(slice_as_array)]
use psp::dprintln;

pub mod assets;
pub mod asset_handler;

pub extern crate alloc;

#[macro_export]
macro_rules! add_asset {
    ($path:literal) => {
        dprintln!("{}", $path);
    };
}
