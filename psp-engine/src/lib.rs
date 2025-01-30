#![no_std]
#![feature(map_try_insert)]
#![feature(vec_into_raw_parts)]
#![feature(slice_as_array)]

pub mod input;
pub mod render;
pub(crate) mod utils;
pub mod gu;
pub mod asset_handling;

pub extern crate alloc;