extern crate alloc;
use alloc::format;
use alloc::string::String;
use core::ffi::c_void;
use core::ptr::{null_mut, slice_from_raw_parts_mut};
use psp::sys;
use psp::sys::{sceGuGetMemory, GuPrimitive, VertexType};
use zero_derive::Zero;

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
    u: f32,
    v: f32,
    color: u32,
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
pub struct Texture<const N: usize> {
    pub width: u32,
    pub height: u32,
    pub data: psp::Align16<[u8; N]>,
}

impl<const N: usize> Texture<N> {

    /// Create a new texture from raw data
    pub fn new_raw(width: u32, height: u32, p: [u8; N]) -> Texture<N> {
        Texture {
            width,
            height,
            data: psp::Align16(p),
        }
    }
}

pub unsafe fn draw_rect<const N: usize>(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: u32,
    texture: &Texture<N>,
) {
    let vert_len = 2;
    let vert_ptr =
        sceGuGetMemory((vert_len * size_of::<TextureVertex>()) as i32) as *mut TextureVertex;
    let vertices = slice_from_raw_parts_mut(vert_ptr, vert_len);

    (*vertices)[0].zero();
    (*vertices)[1].zero();

    // Define vertex 1
    (*vertices)[0].x = x;
    (*vertices)[0].y = y;

    (*vertices)[0].color = color;

    // Define vertex 2
    (*vertices)[1].u = width;
    (*vertices)[1].v = height;
    (*vertices)[1].x = x + width;
    (*vertices)[1].y = y + height;

    (*vertices)[1].color = color;

    // Make sure the texture cache is ready
    sys::sceKernelDcacheWritebackInvalidateAll();

    // Set the texture pixel format to Psm8888 and disable swizzling
    sys::sceGuTexMode(sys::TexturePixelFormat::Psm8888, 0, 0, 0);

    // Set the texture mapping function to replace all fragments
    sys::sceGuTexFunc(sys::TextureEffect::Replace, sys::TextureColorComponent::Rgb);

    // Set texture map
    sys::sceGuTexImage(
        sys::MipmapLevel::None,
        texture.width as i32,
        texture.height as i32,
        texture.width as i32,
        &texture.data as *const psp::Align16<_> as *const _,
    );

    // Enable the 2dtexture state
    sys::sceGuEnable(sys::GuState::Texture2D);

    // Draw primitive
    sys::sceGuDrawArray(
        GuPrimitive::Sprites,
        VertexType::COLOR_8888
            | VertexType::TEXTURE_32BITF
            | VertexType::VERTEX_32BITF
            | VertexType::TRANSFORM_2D,
        2,
        null_mut(),
        vertices as *mut _ as *mut c_void,
    );

    // Disable 2dtexture state
    sys::sceGuDisable(sys::GuState::Texture2D);
}

#[allow(unused)]
pub fn memory_hex(ptr: *const u8, len: usize) -> String {
    let mut hex_string = String::new();
    unsafe {
        let bytes = core::slice::from_raw_parts(ptr, len);
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
