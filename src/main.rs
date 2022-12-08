mod level;
use bevy::{prelude::*, render::camera::WindowOrigin};
use bevy_inspector_egui::WorldInspectorPlugin;
use std::collections::HashMap;

pub const LEVEL_COUNT: usize = 1;
const LEVELS: [&str; LEVEL_COUNT] = [include_str!("levels/level0.txt")];
const TILE_SIZE: f32 = 16.0;
const TILE_MAX_HEIGHT: f32 = 14.0;
const WINDOW_WITDH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

#[derive(Component)]
struct Index(usize, usize);
#[derive(Component)]
struct Name(String);
#[derive(Component)]
struct Player;

#[derive(Debug, Resource)]
struct TilesHandle(Handle<TextureAtlas>);

#[derive(Debug, Resource)]
struct SpritesHandle(Handle<TextureAtlas>);

#[derive(Debug, Resource, Default)]
struct Game {
    level: level::Level,
    map_to_index: HashMap<char, usize>,
    is_fullscreen: bool,
}
const TT: f32 = 8.0;
const TS: f32 = 16.0;
const TM: f32 = 24.0;
const TB: f32 = 32.0;

static ENTITIES_DIM: &[(f32, f32, f32, f32)] = &[
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
    (0.,  TS, TS, TS),
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
    (0.,  TS * 2.0, TS, TS),
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
    (0.,  TS * 3.0,   TS, TM),
    (16., TS * 3.0,   TS, TM),
    (32., TS * 3.0,   TS, TM),
    (48., TS * 3.0,   TS, TM),
    (64., TS * 3.0,   TS, TM),
    (80., TS * 3.0,   TS, TM),
    (96., TS * 3.0,   TS, TM),
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
    (0.,    TT * 9.0, TS, TS),
    (16.,   TT * 9.0, TS, TS),
    (32.,   TT * 9.0, TS, TS),
    (48.,   TT * 9.0, TS, TS),
    (64.,   TT * 9.0, TS, TS),
    (80.,   TT * 9.0, TS, TS),
    (96.,   TT * 9.0, TS, TS),
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
    (0.,    TT * 11.0, TS, TS),
    (16.,   TT * 11.0, TS, TS),
    (32.,   TT * 11.0, TS, TS),
    (48.,   TT * 11.0, TS, TS),
    (64.,   TT * 11.0, TS, TS),
    (80.,   TT * 11.0, TS, TS),
    (96.,   TT * 11.0, TS, TS),
    (112.0, TT * 11.0, TS, TM),
    (128.0, TT * 11.0, TS, TM),
    (144.0, TT * 11.0, TS, TM),
    (160.0, TT * 11.0, TS, TM),
    (176.0, TT * 11.0, TS, TM),
    (192.0, TT * 11.0, TS, TM),
    (208.0, TT * 11.0, TM, TS),
    (224.0, TT * 12.0, TS, TM),


    //7 TINY mario
    (0.,    TT * 13.0, TS, TS),
    (16.,   TT * 13.0, TS, TS),
    (32.,   TT * 13.0, TS, TS),
    (48.,   TT * 13.0, TS, TS),
    (64.,   TT * 13.0, TS, TS),
    (80.,   TT * 13.0, TS, TS),
    (96.,   TT * 13.0, TS, TS),
];

#[allow(dead_code)]
enum EntityTile {
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
    MarioSmallSwim5 = 96
}

fn main() {
    App::new()
        .insert_resource(Game {
            level: level::Level {
                current: 0,
                ..Default::default()
            },
            map_to_index: HashMap::from([
                ('0', 1),
                ('1', 2),
                ('2', 3),
                ('3', 4),
                ('4', 5),
                ('5', 6),
                ('6', 7),
                ('7', 8),
                ('8', 9),
                ('9', 10),
                (':', 11),
                (';', 12),
                ('<', 13),
                ('=', 14),
                ('>', 15),
                ('?', 16),
                ('@', 17),
                ('A', 18),
                ('B', 19),
                ('C', 20),
            ]),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1920.,
                height: 1080.,
                mode: WindowMode::Windowed,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(bevy::window::close_on_esc)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
        .add_startup_system_to_stage(StartupStage::Startup, build_map)
        .add_startup_system_to_stage(StartupStage::Startup, spawn_mario)
        .add_system_to_stage(CoreStage::PreUpdate, toggle_fullscreen)
        .add_system_to_stage(CoreStage::PreUpdate, move_camera)
        .run();
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprites_texture_handle = asset_server.load("textures/sprites.png");
    let mut sprites_texture_atlas =
        TextureAtlas::new_empty(sprites_texture_handle, Vec2::new(29.0 * 8.0, 29.0 * 8.0));

    for sprite_dim in ENTITIES_DIM {
        sprites_texture_atlas.add_texture(Rect::new(
            sprite_dim.0,
            sprite_dim.1,
            sprite_dim.2 + sprite_dim.0,
            sprite_dim.3 + sprite_dim.1,
        ));
    }
    let sprites_texture_atlas_handle = texture_atlases.add(sprites_texture_atlas.clone());
    let tiles_texture_handle = asset_server.load("textures/tiles.png");
    let tiles_texture_atlas = TextureAtlas::from_grid(
        tiles_texture_handle,
        Vec2::new(16.0, 16.0),
        15,
        13,
        None,
        None,
    );
    let tiles_texture_atlas_handle = texture_atlases.add(tiles_texture_atlas);

    commands.insert_resource(TilesHandle(tiles_texture_atlas_handle));
    commands.insert_resource(SpritesHandle(sprites_texture_atlas_handle));

    let scale_factor = WINDOW_HEIGHT / TILE_SIZE / TILE_MAX_HEIGHT;
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(-TILE_SIZE / 2.0, -TILE_SIZE / 2.0, 1.0),
            scale: Vec3::new(1.0 / scale_factor, 1.0 / scale_factor, 2.0),
            ..Default::default()
        },
        projection: OrthographicProjection {
            window_origin: WindowOrigin::BottomLeft,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn build_map(mut commands: Commands, game_resource: Res<Game>, tiles_handle: Res<TilesHandle>) {
    let current_level = level::LevelFile::new(LEVELS[game_resource.level.current]);
    for y in 0..current_level.dims.1 {
        for x in 0..current_level.dims.0 {
            let pos = (x, y);
            let tile = current_level.get(pos);
            let index_map = game_resource.map_to_index.get(&tile).unwrap();
            commands
                .spawn(SpriteSheetBundle {
                    texture_atlas: tiles_handle.0.clone(),
                    transform: Transform::from_xyz((x * 16) as f32, (y * 16) as f32, 0.0),
                    sprite: TextureAtlasSprite::new(*index_map),
                    ..default()
                })
                .insert(Name(String::from(tile)))
                .insert(Index(pos.0, pos.1));

            // match tile {
            //     'C' => {}
            //     _ => {}
            // }
        }
    }
}

fn spawn_mario(
    mut commands: Commands,
    game_resource: Res<Game>,
    sprites_handle: Res<SpritesHandle>,
) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: sprites_handle.0.clone(),
            transform: Transform::from_xyz(32., 32., 1.0),
            sprite: TextureAtlasSprite::new(EntityTile::MarioSmallIdle as usize),
            ..default()
        },
        Player,
    ));
}

fn toggle_fullscreen(
    mut game_resource: ResMut<Game>,
    input: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.primary_mut();
    if input.just_pressed(KeyCode::F12) {
        if game_resource.is_fullscreen {
            window.set_mode(WindowMode::Windowed);
        } else {
            window.set_mode(WindowMode::BorderlessFullscreen);
        }
        game_resource.is_fullscreen = !game_resource.is_fullscreen
    }
}

fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut transform = query.get_single_mut().unwrap();
    if keyboard_input.pressed(KeyCode::Left) {
        transform.translation.x += -10.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        transform.translation.x += 10.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        transform.translation.y += 10.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        transform.translation.y += -10.0;
    }

    if keyboard_input.pressed(KeyCode::Plus) {
        transform.scale *= 0.8;
    }

    if keyboard_input.pressed(KeyCode::Minus) {
        transform.scale *= 1.2;
    }

    if keyboard_input.pressed(KeyCode::Escape) {

    }
}
