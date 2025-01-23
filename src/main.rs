#![feature(map_try_insert)]
#![no_std]
#![no_main]
extern crate alloc;

mod gu;
mod render;
mod asset_handler;
mod utils;

use alloc::string::ToString;
use psp::dprintln;
use crate::gu::Gu;
use crate::asset_handler::*;
use crate::render::Texture;

psp::module!("factory", 1, 1);

const IMAGE_SIZE: usize = 128;
const IMAGE_PIXELS: usize = IMAGE_SIZE * IMAGE_SIZE;
const IMAGE_LAYOUT_SIZE: usize = IMAGE_PIXELS * 4;
fn psp_main() {
    psp::enable_home_button();

    unsafe {

        // TODO: Make this less cumbersome (Macro?
        let mut asset_handler = AssetHandler::new();

        let ferris_handle = asset_handler.add("ms0:/PSP/GAME/Factory/Assets/ferris.bin").unwrap_or_else(|x| {
            dprintln!("{}", x);
            panic!();
        });

        // TODO: Make a new function that does the next two lines in one line.
        let mut ferris = asset_handler.assets[&ferris_handle];
        //ferris.load().unwrap();

        // TODO: Change how type parameters work for the texture creation.
        let ferris_tex = Texture::<IMAGE_LAYOUT_SIZE>::new_ll(IMAGE_SIZE as u32, IMAGE_SIZE as u32, ferris.handle);

        // Allocate pointers for frame buffers in VRAM
        let mut g = Gu::new();

        // Initialize the GU libraries with our frame buffers and instantiate any other parameters
        g.init_gu();
        g.set_clear_color(0xff000000);

        loop {
            // Call the processor to switch to GU Context and clear the screen
            g.start_frame();

            // Get texture from raw data
            // TODO: Figure out how to use io to lazyload images when they are needed for textures
            // let ferris_texture = render::Texture::new_raw(IMAGE_SIZE as u32, IMAGE_SIZE as u32, *include_bytes!("../assets/ferris.bin"));
            // Add a rectangle primitive to the draw list
            render::draw_rect(216.0, 96.0, 128.0, 128.0, 0xFFFFFFFF, &ferris_tex);
            // Switch context and begin executing the draw list
            g.end_frame();
        }
    }
}