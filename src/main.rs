#![no_std]
#![no_main]

mod gu;
mod render;
use crate::gu::Gu;

psp::module!("factory", 1, 1);

const IMAGE_SIZE: usize = 128;

fn psp_main() {
    psp::enable_home_button();

    unsafe {
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
            let ferris_texture = render::Texture::new_raw(IMAGE_SIZE as u32, IMAGE_SIZE as u32, *include_bytes!("../assets/ferris.bin"));
            
            // Add a rectangle primitive to the draw list
            render::draw_rect(216.0, 96.0, 128.0, 128.0, 0xFFFFFFFF, &ferris_texture);

            // Switch context and begin executing the draw list
            g.end_frame();
        }
    }
}
