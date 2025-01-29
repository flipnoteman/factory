#![no_std]
#![feature(map_try_insert)]
#![allow(unsafe_code)]
pub mod input;
pub mod render;
pub(crate) mod utils;
pub mod gu;
pub mod asset_handling;

pub extern crate alloc;