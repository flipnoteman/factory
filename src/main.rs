#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::ToString;

use psp::dprintln;
use psp::sys::CtrlButtons;

use psp_engine::*;
use psp_engine::asset_handling::asset_handler::AssetHandler;
use psp_engine::asset_handling::assets::{Asset, Raw, BMP};
use psp_engine::gu::Gu;
use psp_engine::input::{get_dpad, init_input};
use psp_engine::render::Texture;

psp::module!("factory", 1, 1);

// const IMAGE_SIZE: usize = 128;
// const IMAGE_PIXELS: usize = IMAGE_SIZE * IMAGE_SIZE;
// const IMAGE_LAYOUT_SIZE: usize = IMAGE_PIXELS * 4;

// #[AssetHandler]
// struct GameAssets;

fn psp_main() {

    psp::enable_home_button();

    // TODO: Make this less cumbersome (Macro?)
    let asset_handler = &mut AssetHandler::new();


    // add_asset!(ferris, "ms0:/PSP/GAME/Factory/Assets/ferris.bin");
    let texture_asset = asset_handler.add::<BMP>("ms0:/PSP/GAME/Factory/Assets/coin_1.bmp").unwrap_or_else(|e| {
        dprintln!("{}", e);
        panic!();
    });
    
    let font_asset = asset_handler.add::<BMP>("ms0:/PSP/GAME/Factory/Assets/Fonts/default_font16x16.bmp").unwrap_or_else(|e| {
        dprintln!("{}", e);
        panic!();
    });

    let mut texture_handle = asset_handler.query_mut::<BMP>(texture_asset).unwrap();
    let mut font_handle = asset_handler.query_mut::<BMP>(font_asset).unwrap();

    match texture_handle.load() {
        Ok(_) => {}
        Err(e) => {dprintln!("ferris_handle.load(): {}", e);}
    };
    
    match font_handle.load() {
        Ok(_) => {}
        Err(e) => {dprintln!("font_handle.load(): {}", e);}
    };

//     // TODO: Change how type parameters work for the texture creation.
    let texture = Texture::from(&mut *texture_handle);
    let font = Texture::from(&mut *font_handle);
// 
    // Allocate pointers for frame buffers in VRAM
    let mut g = Gu::new();

    // Initialize the GU libraries with our frame buffers and instantiate any other parameters
    g.init_gu();
    init_input();
    g.set_clear_color(0xff000000);

    let mut x = 216.0;
    let mut y = 96.0;
    let mut index = 0;

    dprintln!("{:?}", font.adj_size);
    
    loop {
        
        // Call the processor to switch to GU Context and clear the screen
        g.start_frame(true);

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


        render::draw_rect(x, y, 32.0, 32.0, index, 0xFFFFFFFF, &texture);
        render::draw_rect(20., 20., 16.0, 16.0, 4, 0xFFFFFFFF, &font);
        
        index += 1;
        
        if index > 11 {
            index = 0;
        }

        // Switch context and begin executing the draw list
        g.end_frame();
    }
}
