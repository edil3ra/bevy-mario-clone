use std::time::Duration;

pub const WINDOW_WIDTH: f32 = 1920.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;

pub const TILE_MAX_HEIGHT: f32 = 15.0;
pub const TILE_SIZE: u32 = 16;
pub const MAP_HEIGHT: u32 = 14;
// pub const GRAVITY: f32 = 1500.0;
pub const GRAVITY: f32 = 100.0;
// pub const GRAVITY: f32 = 0.0;

pub const FRAMES_RECT_PLAYER: [[u32; 4]; 21] = [
    [0, 88, 16, 16],
    [16, 88, 16, 16],
    [32, 88, 16, 16],
    [48, 88, 16, 16],
    [64, 88, 16, 16],
    [80, 88, 16, 16],
    [96, 88, 16, 16],
    [0, 104, 16, 16],
    [16, 104, 16, 16],
    [32, 104, 16, 16],
    [48, 104, 16, 16],
    [64, 104, 16, 16],
    [80, 104, 16, 16],
    [96, 104, 16, 16],
    [112, 88, 16, 32],
    [128, 88, 16, 32],
    [144, 88, 16, 32],
    [160, 88, 16, 32],
    [176, 88, 16, 32],
    [192, 88, 16, 32],
    [0, 120, 16, 32],
];

pub const FRAMES_RECT_GOOMBA_BROWN: [[u32; 4]; 3] =
    [[80, 0, 16, 16], [96, 0, 16, 16], [112, 0, 16, 16]];
