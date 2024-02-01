pub const LEVEL_COUNT: usize = 1;
pub const LEVELS: [&str; LEVEL_COUNT] = [include_str!("levels/level0.txt")];

pub const WINDOW_WIDTH: f32 = 1920.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;

pub const TILE_SIZE: f32 = 16.0;
pub const TILE_MAX_HEIGHT: f32 = 15.0;
pub const TILE_TILES_COLUMN_SIZE: u32 = 16;

const TT: f32 = 8.0;
const TS: f32 = 16.0;
const TM: f32 = 24.0;
const TB: f32 = 32.0;
pub static ENTITIES_DIM: &[(f32, f32, f32, f32)] = &[
    // 1
    (0., 0., TS, TS),
    (16., 0., TS, TS),
    (32., 0., TS, TS),
    (48., 0., TS, TS),
    (64., 0., TS, TS),
    (80., 0., TS, TS),
    (96., 0., TS, TS),
    (112.0, 0., TS, TS),
    (128.0, 0., TS, TS),
    (144.0, 0., TS, TS),
    (160.0, 0., TS, TS),
    (176.0, 0., TS, TS),
    (192.0, 0., TS, TM),
    (208.0, 0., TS, TM),
    (224.0, 0., TS, TM),
    (240.0, 0., TS, TM),
    // 2
    (0., TS, TS, TS),
    (16., TS, TS, TS),
    (32., TS, TS, TS),
    (48., TS, TS, TS),
    (64., TS, TS, TS),
    (80., TS, TS, TS),
    (96., TS, TS, TS),
    (112.0, TS, TS, TS),
    (128.0, TS, TB, TS),
    // (144.0, 16., TS, TS),
    (160.0, TS, TS, TS),
    (176.0, TS, TS, TS),
    //3
    (0., TS * 2.0, TS, TS),
    (16., TS * 2.0, TS, TS),
    (32., TS * 2.0, TS, TS),
    (48., TS * 2.0, TS, TS),
    (64., TS * 2.0, TS, TS),
    (80., TS * 2.0, TS, TS),
    (96., TS * 2.0, TS, TS),
    (112.0, TS * 2.0, TS, TS),
    (128.0, TS * 2.0, TS, TS),
    (144.0, TS * 2.0, TS, TS),
    (160.0, TS * 2.0, TS, TS),
    (176.0, TS * 2.0, TS, TS),
    (192.0, TS * 2.0, TS, TM),
    (208.0, TS * 2.0, TS, TM),
    (224.0, TS * 2.0, TS, TM),
    (240.0, TS * 2.0, TS, TM),
    //4
    (0., TS * 3.0, TS, TM),
    (16., TS * 3.0, TS, TM),
    (32., TS * 3.0, TS, TM),
    (48., TS * 3.0, TS, TM),
    (64., TS * 3.0, TS, TM),
    (80., TS * 3.0, TS, TM),
    (96., TS * 3.0, TS, TM),
    (112.0, TS * 3.0, TS, TM),
    (128.0, TS * 3.0, TS, TM),
    (144.0, TS * 3.0, TS, TM),
    (160.0, TS * 3.0, TS, TM),
    (176.0, TS * 3.0, TS, TS),
    (192.0, TS * 3.0, TS, TM),
    (208.0, TS * 3.0, TS, TM),
    (224.0, TS * 3.0, TS, TM),
    (240.0, TS * 3.0, TS, TM),
    //5
    (0., TT * 9.0, TS, TS),
    (16., TT * 9.0, TS, TS),
    (32., TT * 9.0, TS, TS),
    (48., TT * 9.0, TS, TS),
    (64., TT * 9.0, TS, TS),
    (80., TT * 9.0, TS, TS),
    (96., TT * 9.0, TS, TS),
    (112.0, TT * 9.0, TS, TS),
    (128.0, TT * 9.0, TS, TS),
    (144.0, TT * 9.0, TS, TS),
    (160.0, TT * 9.0, TS, TS),
    (176.0, TT * 8.0, TS, TM),
    (192.0, TT * 9.0, TS, TS),
    (208.0, TT * 9.0, TS, TS),
    (224.0, TT * 9.0, TS, TS),
    (240.0, TT * 9.0, TS, TM),
    //6
    (0., TT * 11.0, TS, TS),
    (16., TT * 11.0, TS, TS),
    (32., TT * 11.0, TS, TS),
    (48., TT * 11.0, TS, TS),
    (64., TT * 11.0, TS, TS),
    (80., TT * 11.0, TS, TS),
    (96., TT * 11.0, TS, TS),
    (112.0, TT * 11.0, TS, TM),
    (128.0, TT * 11.0, TS, TM),
    (144.0, TT * 11.0, TS, TM),
    (160.0, TT * 11.0, TS, TM),
    (176.0, TT * 11.0, TS, TM),
    (192.0, TT * 11.0, TS, TM),
    (208.0, TT * 11.0, TM, TS),
    (224.0, TT * 12.0, TS, TM),
    //7 TINY mario
    (0., TT * 13.0, TS, TS),
    (16., TT * 13.0, TS, TS),
    (32., TT * 13.0, TS, TS),
    (48., TT * 13.0, TS, TS),
    (64., TT * 13.0, TS, TS),
    (80., TT * 13.0, TS, TS),
    (96., TT * 13.0, TS, TS),
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
