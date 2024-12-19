use psp::vram_alloc::{get_vram_allocator, SimpleVramAllocator};
type VramAllocator = SimpleVramAllocator;

use core::ffi::c_void;
use core::ptr::addr_of_mut;
use psp::{sys, BUF_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};
use psp::sys::{ClearBuffer, DepthFunc, DisplayPixelFormat, GuContextType, GuState, TexturePixelFormat};

const DISPLAY_LIST_LENGTH: usize = 0x40000;
static mut LIST: psp::Align16<[u64; DISPLAY_LIST_LENGTH]> = psp::Align16([0; DISPLAY_LIST_LENGTH]);

#[repr(C)]
pub struct Gu {
    allocator: VramAllocator,
    fbp0: *mut u8,
    fbp1: *mut u8,
    zbp: *mut u8,
    clear_color: u32,
}

impl Gu {
    pub unsafe fn new() -> Gu {
        let allocator = get_vram_allocator().unwrap();
        let fbp0 = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888).as_mut_ptr_from_zero();
        let fbp1 = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888).as_mut_ptr_from_zero();
        let zbp = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm4444).as_mut_ptr_from_zero();
        let clear_color = 0xFFFFFFFF;

        Gu {
            allocator,
            fbp0,
            fbp1,
            zbp,
            clear_color
        }
    }

    pub fn set_clear_color(&mut self, color: u32) {
        self.clear_color = color;
    }

    pub unsafe fn init_gu(&mut self) {
        sys::sceGuInit();

        // Set up buffers
        sys::sceGuStart(GuContextType::Direct, addr_of_mut!(LIST) as *mut _ as *mut c_void);
        sys::sceGuDrawBuffer(DisplayPixelFormat::Psm8888, self.fbp0 as _, BUF_WIDTH as i32);
        sys::sceGuDispBuffer(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, self.fbp1 as _, BUF_WIDTH as i32);
        sys::sceGuDepthBuffer(self.zbp as _, BUF_WIDTH as i32);

        // Set up viewport
        sys::sceGuOffset(2048 - (SCREEN_WIDTH / 2), 2048 - (SCREEN_HEIGHT / 2));
        sys::sceGuViewport(2048, 2048, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        sys::sceGuEnable(GuState::ScissorTest);
        sys::sceGuScissor(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);

        // Set up depth
        sys::sceGuDepthRange(65535, 0); // Use full buffer for depth testing
        sys::sceGuDepthFunc(DepthFunc::GreaterOrEqual); // Depth buffer is reversed so Greater than or equals
        sys::sceGuEnable(GuState::DepthTest); // Enable depth testing

        sys::sceGuFinish();
        sys::sceGuDisplay(true);
    }

    // Switch to gu context, start adding commands and clear screen
    pub unsafe fn start_frame(&self) {
        // Switch to GU context
        sys::sceGuStart(GuContextType::Direct, addr_of_mut!(LIST) as *mut _ as *mut c_void);
        // Clear screen
        sys::sceGuClearColor(self.clear_color);
        sys::sceGuClear(ClearBuffer::COLOR_BUFFER_BIT);
    }

    /// Stop adding commands to display list, swap frames
    pub unsafe fn end_frame(&self) {
        // Finish with GU Context setup, start executing list of commands and swap to parent context
        sys::sceGuFinish();

        // Stalls until display list is finished executing
        sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);

        // Wait for vertical blank start
        sys::sceDisplayWaitVblankStart();

        // Swap display buffer and draw buffer
        sys::sceGuSwapBuffers();
    }
}

