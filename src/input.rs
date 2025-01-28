use alloc::format;
use psp;
use psp::dprintln;
use psp::sys::{sceCtrlReadBufferPositive, sceCtrlSetSamplingCycle, sceCtrlSetSamplingMode, CtrlButtons, CtrlMode, SceCtrlData};
use crate::render;

pub unsafe fn get_dpad() -> CtrlButtons {

    let mut pad = SceCtrlData::default();
    sceCtrlReadBufferPositive(&mut pad, 1);

    let d_pad = CtrlButtons::UP | CtrlButtons::DOWN | CtrlButtons::LEFT | CtrlButtons::RIGHT;

    pad.buttons & d_pad
}

pub unsafe fn init_input() {
    sceCtrlSetSamplingCycle(0);
    sceCtrlSetSamplingMode(CtrlMode::Analog);
}