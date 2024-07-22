mod config;
mod debug;
// mod level;
mod map;
use bevy::asset::Asset;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;

use bevy::render::render_resource::Texture;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_egui::egui::TextBuffer;
use debug::DebugPlugins;
use map::{MapPlugins, TileFactory, TileType};
use serde::Deserialize;
use std::collections::HashMap;
use std::ops::Add;

#[derive(Component)]
struct Player;

// #[derive(Debug, Default)]
// pub struct AssetsHandle {
//     texture_entities: Handle<Image>,
//     texture_tiles: Handle<Image>,
//     entities_texture_atlas: Handle<TextureAtlasLayout>,
// }

#[derive(Debug, Default)]
pub struct AssetsHandle {
    textures: HashMap<String, Handle<Image>>,
    levels: HashMap<String, Handle<Level>>,
    sprites: HashMap<String, Handle<Sprite>>,
    patterns: HashMap<String, Handle<Pattern>>,
}

#[derive(Debug, Default, Resource)]
pub struct Game {
    assets: AssetsHandle,
    current_level: String,
    is_fullscreen: bool,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Load,
    InGame,
}

#[derive(Reflect, Debug)]
pub enum Direction {
    Idle,
    Left,
    Right,
}

#[derive(Reflect, Default, Debug, Component)]
#[reflect(Component)]
pub struct Physics {
    acceleration: Vec3,
    velocity: Vec3,
    // is_ground: bool,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
struct Action {
    direction: Direction,
    jump: bool,
}
impl Action {
    fn reset(&mut self) {
        self.direction = Direction::default();
        self.jump = false;
    }
}

#[derive(Deserialize, Asset, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct Level {
    sprite_sheet: String,
    pattern_sheet: String,
    music_sheet: String,
    checkpoints: Vec<[u32; 2]>,
    layers: Vec<LevelLayer>,
    entities: Vec<LevelEntity>,
    triggers: Vec<LevelTrigger>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct LevelLayer {
    tiles: Vec<LevelTile>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct LevelTile {
    style: Option<String>,
    pattern: Option<String>,
    behavior: Option<String>,
    ranges: Vec<Vec<i32>>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct LevelEntity {
    name: String,
    pos: [u32; 2],
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct LevelTrigger {
    action: String,
    name: String,
    pos: [u32; 2],
}

#[derive(Deserialize, Asset, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct Sprite {
    image_url: String,
    tile_w: u8,
    tile_h: u8,
    tiles: Vec<SpriteTile>,
    animations: Vec<Animation>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct SpriteTile {
    name: String,
    index: Option<[u8; 2]>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct Animation {
    name: String,
    frame_len: f32,
    frames: Vec<String>,
}

#[derive(Deserialize, Asset, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct Pattern {
    patterns: HashMap<String, PatternTiles>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct PatternTiles {
    tiles: Vec<PatternTile>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct PatternTile {
    style: Option<String>,
    behaviour: Option<String>,
    ranges: Vec<Vec<u32>>,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (config::WINDOW_WIDTH, config::WINDOW_HEIGHT).into(),
                        mode: bevy::window::WindowMode::Windowed,
                        title: "Mario".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            JsonAssetPlugin::<Level>::new(&["level.json"]),
            JsonAssetPlugin::<Pattern>::new(&["pattern.json"]),
            JsonAssetPlugin::<Sprite>::new(&["sprite.json"]),
            MapPlugins {},
            DebugPlugins {},
        ))
        .init_state::<AppState>()
        .register_type::<Action>()
        .register_type::<Physics>()
        .insert_resource(Game {
            current_level: "1-1".to_string(),
            ..Default::default()
        })
        .add_systems(Startup, (load_assets, setup).chain())
        .add_systems(OnEnter(AppState::InGame), (spawn_camera, spawn_mario))
        .add_systems(
            Update,
            (
                sync_player_action_with_input,
                update_player,
                update_physics,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .run();
}

fn load_assets(
    mut game_res: ResMut<Game>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (name, url) in config::TEXTURES.iter() {
        game_res
            .assets
            .textures
            .insert(name.to_string(), asset_server.load(url.to_string()));
    }

    for (name, url) in config::LEVELS.iter() {
        game_res
            .assets
            .levels
            .insert(name.to_string(), asset_server.load(url.to_string()));
    }

    for (name, url) in config::PATTERNS.iter() {
        game_res
            .assets
            .patterns
            .insert(name.to_string(), asset_server.load(url.to_string()));
    }

    for (name, url) in config::SPRITES.iter() {
        game_res
            .assets
            .sprites
            .insert(name.to_string(), asset_server.load(url.to_string()));
    }

    let mut entities_atlas = TextureAtlasLayout::new_empty(UVec2::new(32 * 8, 32 * 8));

    for entities_dim in config::ENTITIES_DIM {
        entities_atlas.add_texture(URect::new(
            entities_dim.0,
            entities_dim.1,
            entities_dim.2 + entities_dim.0,
            entities_dim.3 + entities_dim.1,
        ));
    }

    let sprites_texture_atlas_handle = texture_atlases.add(entities_atlas.clone());

    
}

fn setup(mut game_res: ResMut<Game>, mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame);
}

fn spawn_camera(mut commands: Commands) {
    let scale_factor = config::WINDOW_HEIGHT / config::TILE_SIZE / config::TILE_MAX_HEIGHT;
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(
                (config::WINDOW_WIDTH - config::TILE_SIZE * 2.0) / scale_factor / 2.0
                    - (config::TILE_SIZE / 2.0),
                (config::WINDOW_HEIGHT - config::TILE_SIZE * 2.0) / scale_factor / 2.0
                    - (config::TILE_SIZE / 2.0),
                1.0,
            ),
            scale: Vec3::new(1.0 / scale_factor, 1.0 / scale_factor, 2.0),
            ..Default::default()
        },
        projection: OrthographicProjection {
            ..Default::default()
        },
        ..Default::default()
    });
}

// fn spawn_mario(mut commands: Commands, game_resource: Res<Game>) {
//     let init_position = Vec2::new(32.0, 32.0);
//     commands.spawn((
//         SpriteSheetBundle {
//             texture: game_resource.assets.texture_tiles.clone(),
//             atlas: TextureAtlas {
//                 layout: game_resource.assets.entities_texture_atlas.clone(),
//                 index: config::EntityTile::MarioSmallIdle as usize,
//             },
//             transform: Transform::from_xyz(init_position.x, init_position.y, 1.0),
//             ..default()
//         },
//         Name::new("mario"),
//         Player,
//         Action {
//             direction: Direction::Idle,
//             jump: false,
//         },
//         Physics {
//             ..Default::default()
//         },
//     ));
// }

fn update_player(time: Res<Time>, mut query: Query<(&mut Physics, &Action), With<Player>>) {
    let dt = time.delta().as_secs_f32();
    for (mut physics, intention) in &mut query {
        let abs_x = physics.velocity.x.abs();
        let mut distance = 0.0; // will be used later
        let direction = match intention.direction {
            Direction::Idle => 0,
            Direction::Left => -1,
            Direction::Right => 1,
        };

        if direction == 0 {
            if physics.velocity.x != 0.0 {
                let decel = abs_x.min(300.0 * dt);
                if physics.velocity.x > 0.0 {
                    physics.velocity.x -= decel;
                } else {
                    physics.velocity.x += decel;
                }
            } else {
                distance = 0.0;
            }
        } else {
            physics.velocity.x += 400.0 * direction as f32 * dt;
        }
        let drag = 1.0 / 5000.0 * physics.velocity.x * abs_x;
        physics.velocity.x -= drag;
        distance = abs_x * dt;
    }
}

fn update_physics(
    mut last_time: Local<f32>,
    time: Res<Time>,
    mut query: Query<(&mut Physics, &mut Transform)>,
    mut game_res: ResMut<Game>,
    levels: Res<Assets<Level>>,
) {
    let t = levels.iter();
    let dt = time.elapsed_seconds() - *last_time;
    for (physics, mut transform) in &mut query {
        transform.translation = transform.translation.add(physics.velocity * dt);
    }
    *last_time = time.elapsed_seconds();
}

fn sync_player_action_with_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut action_query: Query<&mut Action, With<Player>>,
) {
    let mut intention = action_query.get_single_mut().unwrap();
    intention.reset();

    if keyboard_input.pressed(KeyCode::ArrowLeft) && keyboard_input.pressed(KeyCode::ArrowRight) {
        intention.direction = Direction::Idle;
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
        intention.direction = Direction::Left;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        intention.direction = Direction::Right;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        intention.jump = true
    }
}
