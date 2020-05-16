use super::*;

pub const LOAD_SCENE_BTN: usize = 0;
pub const START_BTN: usize = 1;
pub const PAUSE_BTN: usize = 2;
pub const SAVE_BTN: usize = 3;

pub const WIDTH: u32 = 512;
pub const HEIGHT: u32 = 384;
pub const FPS_LIMIT: Float = 10.;
pub const PHAROSA: &str = "Pharosa";

pub const WINDOW_OPTS: WindowOptions = WindowOptions {
    borderless: false,
    title: true,
    resize: true,
    scale: Scale::X2,
    scale_mode: ScaleMode::AspectRatioStretch,
    topmost: false,
};

pub const SAVE_PATH: &str = "result.png";
