use alloc::format;
use alloc::string::String;
use core::ffi::c_void;
use core::ptr::{null_mut, slice_from_raw_parts_mut};
use core::slice;
use embedded_graphics::geometry::Dimensions;
use embedded_graphics::pixelcolor::Bgr888;
use psp::sys;
use psp::sys::{sceGuGetMemory, GuPrimitive, VertexType};
use zero_derive::Zero;

use tinybmp::Bmp;

#[repr(C)]
#[derive(Debug, Zero)]
struct Vertex {
    u: u16,
    v: u16,
    x: i16,
    y: i16,
    z: i16,
}

#[repr(C)]
#[derive(Debug, Zero)]
struct TextureVertex {
    u: u16,
    v: u16,
    color: u32,
    x: i16,
    y: i16,
    z: i16,
}

#[repr(C)]
pub struct Texture {
    width: u32,
    height: u32,
    data: *const [u8],
}

pub unsafe fn load_texture(filename: &[u8]) -> Texture {
    let bmp: Bmp<Bgr888> = Bmp::from_slice(filename).unwrap();
    let (width, height) = (bmp.bounding_box().size.width, bmp.bounding_box().size.height);

    let buffer = bmp.as_raw().image_data();

    Texture {
        width,
        height,
        data: buffer,
    }
}

pub unsafe fn draw_rect(x: i16, y: i16, width: i16, height: i16, color: u32) {
    let vert_len = 3;
    let vert_ptr = sceGuGetMemory((vert_len * size_of::<TextureVertex>()) as i32) as *mut TextureVertex;
    let vertices = slice_from_raw_parts_mut(vert_ptr, vert_len);

    (*vertices)[0].zero();
    (*vertices)[1].zero();
    (*vertices)[2].zero();

    (*vertices)[0].x = x;
    (*vertices)[0].y = y;

    (*vertices)[0].color = color;

    (*vertices)[1].x = x + width;
    (*vertices)[1].y = y + height;

    (*vertices)[1].color = color;
    // sys::sceGuDebugPrint(0, 0, 0x00FFFF00, format!("Vertices: \n{}", memory_hex(vertices as *const _ as *const u8, 2 * size_of::<Vertex>())).as_ptr());
    // sys::sceGuDebugFlush();
    sys::sceGuDrawArray(GuPrimitive::Sprites, VertexType::COLOR_8888 | VertexType::TEXTURE_16BIT | VertexType::VERTEX_16BIT | VertexType::TRANSFORM_2D, 3, null_mut(), vertices as *mut _ as *mut c_void);
}

#[allow(unused)]
pub fn memory_hex(ptr: *const u8, len: usize) -> String{
    let mut hex_string = String::new();
    unsafe {
        let bytes = slice::from_raw_parts(ptr, len);
        for (i, byte) in bytes.iter().enumerate() {
            hex_string.push_str(&format!("{:02X} ", byte));
            if (i + 1) % 4 == 0 {
                hex_string.push('\n'); // New line every 16 bytes
            }
        }
        if len % 4 != 0 {
            hex_string.push('\n'); // Ensure a new line at the end
        }
        hex_string
    }
}