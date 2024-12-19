#![no_std]
#![no_main]

mod gu;
mod render;
extern crate alloc;
use crate::gu::Gu;

psp::module!("factory", 1, 1);

fn psp_main() {
    psp::enable_home_button();

    unsafe {
        // Allocate pointers for frame buffers in VRAM
        let mut g = Gu::new();
        // Initialize the GU libraries with our frame buffers and instantiate any other parameters
        g.init_gu();
        g.set_clear_color(0xff000000);

        loop{
            // Call the processor to switch to GU Context and clear the screen
            g.start_frame();
            render::load_texture(include_bytes!("../assets/ferris.bmp"));
            // Add a rectangle primitive to the draw list
            render::draw_rect(216, 96, 34, 64, 0xFF00FF00);

            // Switch context and begin executing teh draw list
            g.end_frame();
        }
    }
}
