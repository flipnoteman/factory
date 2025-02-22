#![no_std]
#![feature(map_try_insert)]
#![feature(vec_into_raw_parts)]
#![feature(slice_as_array)]

pub mod input;
pub mod render;
pub mod gu;

pub extern crate alloc;
pub extern crate misc;
pub extern crate asset_handling;
