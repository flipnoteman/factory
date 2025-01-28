#![feature(map_try_insert)]
#![no_std]
#![no_main]
extern crate alloc;

mod gu;
mod render;
mod asset_handling;
mod utils;
mod input;

use alloc::boxed::Box;
use alloc::string::ToString;
use psp::dprintln;
use psp::sys::CtrlButtons;
use crate::gu::Gu;
use asset_handling::assets::*;
use crate::render::Texture;
// use asset_macros::AssetHandler;
use crate::asset_handling::asset_handler::AssetHandler;
use crate::input::*;

psp::module!("factory", 1, 1);

const IMAGE_SIZE: usize = 128;
const IMAGE_PIXELS: usize = IMAGE_SIZE * IMAGE_SIZE;
const IMAGE_LAYOUT_SIZE: usize = IMAGE_PIXELS * 4;

// #[AssetHandler]
// struct GameAssets;

fn psp_main() {

    psp::enable_home_button();

    unsafe {
        // TODO: Make this less cumbersome (Macro?)
        let mut asset_handler = AssetHandler::new();


        // add_asset!(ferris, "ms0:/PSP/GAME/Factory/Assets/ferris.bin");
        //
        let ferris_handle = asset_handler.add::<Raw>("ms0:/PSP/GAME/Factory/Assets/ferris.bin").unwrap_or_else(|e| {
            dprintln!("{}", e);
            panic!();
        });

        let ferris = asset_handler.query_mut::<Raw>(ferris_handle).unwrap();
        ferris.load().expect("TODO: panic message");

        // TODO: Change how type parameters work for the texture creation.
        let ferris_tex = Texture::<IMAGE_LAYOUT_SIZE>::new_from_raw_ptr(IMAGE_SIZE as u32, IMAGE_SIZE as u32, ferris.handle.unwrap());

        // Allocate pointers for frame buffers in VRAM
        let mut g = Gu::new();

        // Initialize the GU libraries with our frame buffers and instantiate any other parameters
        g.init_gu();
        init_input();
        g.set_clear_color(0xff000000);

        let mut x = 216.0;
        let mut y = 96.0;

        loop {
            // Call the processor to switch to GU Context and clear the screen
            g.start_frame();

            // Get texture from raw data
            // TODO: Figure out how to use io to lazyload images when they are needed for textures
            // let ferris_texture = render::Texture::new_raw(IMAGE_SIZE as u32, IMAGE_SIZE as u32, *include_bytes!("../asset_handling/ferris.bin"));
            // Add a rectangle primitive to the draw list

            let input = get_dpad();

            if (input & CtrlButtons::LEFT).bits() > 0 {
                x -= 1.0;
            }

            if (input & CtrlButtons::RIGHT).bits() > 0 {
                x += 1.0;
            }

            if (input & CtrlButtons::UP).bits() > 0 {
                y -= 1.0;
            }

            if (input & CtrlButtons::DOWN).bits() > 0 {
                y += 1.0;
            }

            render::draw_rect(x, y, 128.0, 128.0, 0xFFFFFFFF, &ferris_tex);
            // Switch context and begin executing the draw list
            g.end_frame();
        }
    }
}