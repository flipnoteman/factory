#![no_std]
#![feature(slice_as_array)]
#![feature(map_try_insert)]

extern crate alloc;
pub mod handler;
pub mod assets;

#[macro_export]
macro_rules! add_asset {
    ($path:literal) => {
        dprintln!("{}", $path);
    };
}
