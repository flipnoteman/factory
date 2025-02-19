use psp;
use psp::sys::{
    CtrlButtons, CtrlMode, SceCtrlData, sceCtrlReadBufferPositive, sceCtrlSetSamplingCycle,
    sceCtrlSetSamplingMode,
};

pub fn get_dpad() -> CtrlButtons {
    unsafe {
        let mut pad = SceCtrlData::default();
        sceCtrlReadBufferPositive(&mut pad, 1);

        let d_pad = CtrlButtons::UP | CtrlButtons::DOWN | CtrlButtons::LEFT | CtrlButtons::RIGHT;

        pad.buttons & d_pad
    }
}

pub fn init_input() {
    unsafe {
        sceCtrlSetSamplingCycle(0);
        sceCtrlSetSamplingMode(CtrlMode::Analog);
    }
}
