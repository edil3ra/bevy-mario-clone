pub const LEVEL_COUNT: usize = 1;
pub const LEVELS: [&str; LEVEL_COUNT] = [include_str!("levels/level0.txt")];

pub const WINDOW_WIDTH: f32 = 1920.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;

pub const TILE_SIZE: f32 = 16.0;
pub const TILE_MAX_HEIGHT: f32 = 15.0;
pub const TILE_TILES_COLUMN_SIZE: u32 = 16;

const TT: u32 = 8;
const TS: u32 = 16;
const TM: u32 = 24;
const TB: u32 = 32;
pub static ENTITIES_DIM: &[(u32, u32, u32, u32)] = &[
    // 1
    (0, 0, TS, TS),
    (16, 0, TS, TS),
    (32, 0, TS, TS),
    (48, 0, TS, TS),
    (64, 0, TS, TS),
    (80, 0, TS, TS),
    (96, 0, TS, TS),
    (112, 0, TS, TS),
    (128, 0, TS, TS),
    (144, 0, TS, TS),
    (160, 0, TS, TS),
    (176, 0, TS, TS),
    (192, 0, TS, TM),
    (208, 0, TS, TM),
    (224, 0, TS, TM),
    (240, 0, TS, TM),
    // 2
    (0, TS, TS, TS),
    (16, TS, TS, TS),
    (32, TS, TS, TS),
    (48, TS, TS, TS),
    (64, TS, TS, TS),
    (80, TS, TS, TS),
    (96, TS, TS, TS),
    (112, TS, TS, TS),
    (128, TS, TB, TS),
    // (144, 16, TS, TS),
    (160, TS, TS, TS),
    (176, TS, TS, TS),
    //3
    (0, TS * 2, TS, TS),
    (16, TS * 2, TS, TS),
    (32, TS * 2, TS, TS),
    (48, TS * 2, TS, TS),
    (64, TS * 2, TS, TS),
    (80, TS * 2, TS, TS),
    (96, TS * 2, TS, TS),
    (112, TS * 2, TS, TS),
    (128, TS * 2, TS, TS),
    (144, TS * 2, TS, TS),
    (160, TS * 2, TS, TS),
    (176, TS * 2, TS, TS),
    (192, TS * 2, TS, TM),
    (208, TS * 2, TS, TM),
    (224, TS * 2, TS, TM),
    (240, TS * 2, TS, TM),
    //4
    (0, TS * 3, TS, TM),
    (16, TS * 3, TS, TM),
    (32, TS * 3, TS, TM),
    (48, TS * 3, TS, TM),
    (64, TS * 3, TS, TM),
    (80, TS * 3, TS, TM),
    (96, TS * 3, TS, TM),
    (112, TS * 3, TS, TM),
    (128, TS * 3, TS, TM),
    (144, TS * 3, TS, TM),
    (160, TS * 3, TS, TM),
    (176, TS * 3, TS, TS),
    (192, TS * 3, TS, TM),
    (208, TS * 3, TS, TM),
    (224, TS * 3, TS, TM),
    (240, TS * 3, TS, TM),
    //5
    (0, TT * 9, TS, TS),
    (16, TT * 9, TS, TS),
    (32, TT * 9, TS, TS),
    (48, TT * 9, TS, TS),
    (64, TT * 9, TS, TS),
    (80, TT * 9, TS, TS),
    (96, TT * 9, TS, TS),
    (112, TT * 9, TS, TS),
    (128, TT * 9, TS, TS),
    (144, TT * 9, TS, TS),
    (160, TT * 9, TS, TS),
    (176, TT * 8, TS, TM),
    (192, TT * 9, TS, TS),
    (208, TT * 9, TS, TS),
    (224, TT * 9, TS, TS),
    (240, TT * 9, TS, TM),
    //6
    (0, TT * 11, TS, TS),
    (16, TT * 11, TS, TS),
    (32, TT * 11, TS, TS),
    (48, TT * 11, TS, TS),
    (64, TT * 11, TS, TS),
    (80, TT * 11, TS, TS),
    (96, TT * 11, TS, TS),
    (112, TT * 11, TS, TM),
    (128, TT * 11, TS, TM),
    (144, TT * 11, TS, TM),
    (160, TT * 11, TS, TM),
    (176, TT * 11, TS, TM),
    (192, TT * 11, TS, TM),
    (208, TT * 11, TM, TS),
    (224, TT * 12, TS, TM),
    //7 TINY mario
    (0, TT * 13, TS, TS),
    (16, TT * 13, TS, TS),
    (32, TT * 13, TS, TS),
    (48, TT * 13, TS, TS),
    (64, TT * 13, TS, TS),
    (80, TT * 13, TS, TS),
    (96, TT * 13, TS, TS),
];

#[allow(dead_code)]
pub enum EntityTile {
    BuzzyBeetle1 = 0,
    BuzzyBeetle2 = 1,
    BuzzyBeetle3 = 2,

    MarioSmallIdle = 75,
    MarioSmallRun1 = 76,
    MarioSmallRun2 = 77,
    MarioSmallJump1 = 78,
    MarioSmallJump2 = 79,
    MarioSmallJump3 = 80,
    MarioSmallDead = 81,

    MarioSmallSwim1 = 92,
    MarioSmallSwim2 = 93,
    MarioSmallSwim3 = 94,
    MarioSmallSwim4 = 95,
    MarioSmallSwim5 = 96,
}
