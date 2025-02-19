#![allow(unused)]

extern crate alloc;
use alloc::format;
use alloc::string::String;
use core::ffi::c_void;
use core::ptr::{null_mut, slice_from_raw_parts_mut};
use psp::sys;
use psp::sys::{GuPrimitive, VertexType, sceGuGetMemory};
use zero_derive::Zero;

use asset_handling::assets::BMP;
use misc::{convert_ptwo, is_pow_two};

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

#[repr(C, align(16))]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub adj_size: (u32, u32),
    pub data: *mut c_void,
}

impl Texture {
    /// Create a new texture from raw data
    pub fn new_from_raw(width: u32, height: u32, p: &[u8]) -> Texture {
        Texture {
            width,
            height,
            adj_size: convert_ptwo(width, height),
            data: p.as_ptr() as *mut c_void,
        }
    }

    pub fn new_from_raw_ptr(width: u32, height: u32, data: *mut c_void) -> Texture {
        Texture {
            width,
            height,
            adj_size: convert_ptwo(width, height),
            data,
        }
    }
}

impl From<BMP> for Texture {
    fn from(value: BMP) -> Self {
        unsafe {
            // Get a pointer to the actual data of the bmp
            let d_ptr = value.handle.unwrap();

            Texture {
                width: value.bih.width,
                height: value.bih.height,
                adj_size: convert_ptwo(value.bih.width, value.bih.height),
                data: d_ptr,
            }
        }
    }
}

impl From<&mut BMP> for Texture {
    fn from(value: &mut BMP) -> Self {
        unsafe {
            let d_ptr = value.handle.unwrap();
            let w = value.bih.width;
            let h = value.bih.height;

            Texture {
                width: w,
                height: h,
                adj_size: convert_ptwo(w, h),
                data: d_ptr,
            }
        }
    }
}

pub fn draw_rect(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    index: u32,
    color: u32,
    texture: &Texture,
) {
    unsafe {
        let vert_len = 2;
        let vert_ptr =
            sceGuGetMemory((vert_len * size_of::<TextureVertex>()) as i32) as *mut TextureVertex;
        let vertices = slice_from_raw_parts_mut(vert_ptr, vert_len);

        (*vertices)[0].zero();
        (*vertices)[1].zero();

        let tex_w = texture.width / width as u32;
        let tex_h = texture.height / height as u32;
        let mut x_ind = index % tex_w;
        let mut y_ind = index / tex_w;

        let x_offset = x_ind as f32 * width;
        let y_offset = y_ind as f32 * height;

        // Define vertex 1
        (*vertices)[0].u = 0. + x_offset;
        (*vertices)[0].v = 0. + y_offset;
        (*vertices)[0].x = x;
        (*vertices)[0].y = y;

        (*vertices)[0].color = color;

        // Define vertex 2
        (*vertices)[1].u = width + x_offset;
        (*vertices)[1].v = height + y_offset;
        (*vertices)[1].x = x + width;
        (*vertices)[1].y = y + height;

        (*vertices)[1].color = color;

        // Make sure the texture cache is ready
        sys::sceKernelDcacheWritebackInvalidateAll();

        // Set the texture pixel format to Psm8888 and disable swizzling
        sys::sceGuTexMode(sys::TexturePixelFormat::Psm8888, 0, 0, 0);

        // Set the texture mapping function to replace all fragments
        sys::sceGuTexFunc(
            sys::TextureEffect::Replace,
            sys::TextureColorComponent::Rgba,
        );

        // Set texture map
        sys::sceGuTexImage(
            sys::MipmapLevel::None,
            texture.adj_size.0 as i32, // Needs to be power of 2
            texture.adj_size.1 as i32, // Needs to be power of 2
            texture.width as i32,      // Needs to a multiple of 4
            texture.data,
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
