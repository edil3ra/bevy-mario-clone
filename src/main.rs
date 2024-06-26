mod config;
mod debug;
mod level;
mod map;
use bevy::prelude::*;

use debug::DebugPlugins;
use map::{MapPlugins, TileFactory, TileType};
use std::collections::HashMap;

#[derive(Component)]
struct Player;

#[derive(Debug, Default)]
pub struct AssetsHandle {
    tiles_image: Handle<Image>,
    entities_image: Handle<Image>,
    entities_texture_atlas: Handle<TextureAtlasLayout>,
}

#[derive(Debug, Default, Resource)]
pub struct Game {
    assets: AssetsHandle,
    level: level::Level,
    map_char_to_texture_index: HashMap<char, TileFactory>,
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
    force: Vec2,
    mass: f32,
    acceleration: Vec2,
    velocity: Vec2,
    position: Vec2,
    is_ground: bool,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
struct Intention {
    direction: Direction,
    jump: bool,
}
impl Intention {
    fn reset(&mut self) {
        self.direction = Direction::default();
        self.jump = false;
    }
}

fn main() {
    App::new()
        .init_state::<AppState>()
        .register_type::<Intention>()
        .register_type::<Physics>()
        .insert_resource(Game {
            level: level::Level {
                current: 0,
                ..Default::default()
            },
            map_char_to_texture_index: HashMap::from([
                (
                    '0',
                    TileFactory {
                        name: String::from("unkown_for_now"),
                        tileType: TileType::Fixed { texture_index: 100 },
                        ..Default::default()
                    },
                ),
                (
                    '1',
                    TileFactory {
                        name: String::from("name"),
                        tileType: TileType::Fixed { texture_index: 2 },
                        ..Default::default()
                    },
                ),
                (
                    '2',
                    TileFactory {
                        name: String::from("name"),
                        tileType: TileType::Fixed { texture_index: 3 },
                        ..Default::default()
                    },
                ),
                (
                    '3',
                    TileFactory {
                        name: String::from("name"),
                        tileType: TileType::Fixed { texture_index: 4 },
                        ..Default::default()
                    },
                ),
                (
                    '4',
                    TileFactory {
                        name: String::from("name"),
                        tileType: TileType::Fixed { texture_index: 5 },
                        ..Default::default()
                    },
                ),
                (
                    '5',
                    TileFactory {
                        name: String::from("removable_block"),
                        tileType: TileType::Fixed { texture_index: 1 },
                        ..Default::default()
                    },
                ),
                (
                    '6',
                    TileFactory {
                        name: String::from("name"),
                        tileType: TileType::Fixed { texture_index: 7 },
                        ..Default::default()
                    },
                ),
                (
                    '7',
                    TileFactory {
                        name: String::from("name"),
                        tileType: TileType::Fixed { texture_index: 8 },
                        ..Default::default()
                    },
                ),
                (
                    '8',
                    TileFactory {
                        name: String::from("name"),
                        tileType: TileType::Fixed { texture_index: 9 },
                        ..Default::default()
                    },
                ),
                (
                    '9',
                    TileFactory {
                        name: String::from("name"),
                        tileType: TileType::Fixed { texture_index: 10 },
                        ..Default::default()
                    },
                ),
                (
                    ':',
                    TileFactory {
                        name: String::from("hard_block_brown"),
                        tileType: TileType::Fixed { texture_index: 0 },
                        ..Default::default()
                    },
                ),
                (
                    ';',
                    TileFactory {
                        name: String::from("hard_block_brown"),
                        tileType: TileType::Fixed { texture_index: 0 },
                        ..Default::default()
                    },
                ),
                (
                    '<',
                    TileFactory {
                        name: String::from("name"),
                        tileType: TileType::Fixed { texture_index: 13 },
                        ..Default::default()
                    },
                ),
                (
                    '=',
                    TileFactory {
                        name: String::from("name"),
                        tileType: TileType::Fixed { texture_index: 14 },
                        ..Default::default()
                    },
                ),
                (
                    '>',
                    TileFactory {
                        name: String::from("interogation_block"),
                        tileType: TileType::Fixed { texture_index: 4 },
                        ..Default::default()
                    },
                ),
                (
                    '?',
                    TileFactory {
                        name: String::from("name"),
                        tileType: TileType::Fixed { texture_index: 16 },
                        ..Default::default()
                    },
                ),
                (
                    '@',
                    TileFactory {
                        name: String::from("name"),
                        tileType: TileType::Fixed { texture_index: 17 },
                        ..Default::default()
                    },
                ),
                (
                    'A',
                    TileFactory {
                        name: String::from("name"),
                        tileType: TileType::Fixed { texture_index: 18 },
                        ..Default::default()
                    },
                ),
                (
                    'B',
                    TileFactory {
                        name: String::from("name"),
                        tileType: TileType::Fixed { texture_index: 19 },
                        ..Default::default()
                    },
                ),
                (
                    'C',
                    TileFactory {
                        name: String::from("sky"),
                        tileType: TileType::Fixed {
                            texture_index: 7 * config::TILE_TILES_COLUMN_SIZE + 9,
                        },
                        ..Default::default()
                    },
                ),
            ]),
            ..Default::default()
        })
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
            MapPlugins {},
            DebugPlugins {},
        ))
        .add_systems(Startup, load_assets)
        .add_systems(OnEnter(AppState::InGame), (spawn_camera, spawn_mario))
        .add_systems(
            Update,
            (
                mario_controller,
                update_net_force,
                update_acceleration,
                update_velocity,
                update_position,
                sync_physics,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .run();
}

fn load_assets(
    mut game_res: ResMut<Game>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let sprites_texture_handle: Handle<Image> = asset_server.load("textures/entities.png");

    let mut sprites_texture_atlas =
        TextureAtlasLayout::new_empty(Vec2::new(32.0 * 8.0, 32.0 * 8.0));

    for sprite_dim in config::ENTITIES_DIM {
        sprites_texture_atlas.add_texture(Rect::new(
            sprite_dim.0,
            sprite_dim.1,
            sprite_dim.2 + sprite_dim.0,
            sprite_dim.3 + sprite_dim.1,
        ));
    }

    let sprites_texture_atlas_handle = texture_atlases.add(sprites_texture_atlas.clone());
    let tiles_texture_handle = asset_server.load("textures/tiles.png");

    game_res.assets.tiles_image = tiles_texture_handle;
    game_res.assets.entities_image = sprites_texture_handle;
    game_res.assets.entities_texture_atlas = sprites_texture_atlas_handle;

    next_state.set(AppState::InGame);
}

fn spawn_camera(mut commands: Commands) {
    let scale_factor = config::WINDOW_HEIGHT / config::TILE_SIZE / config::TILE_MAX_HEIGHT;
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(
                (config::WINDOW_WIDTH - config::TILE_SIZE) / scale_factor / 2.0,
                (config::WINDOW_HEIGHT - config::TILE_SIZE) / scale_factor / 2.0,
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

fn spawn_mario(mut commands: Commands, game_resource: Res<Game>) {
    let init_position = Vec2::new(32.0, 32.0);
    commands.spawn((
        SpriteSheetBundle {
            texture: game_resource.assets.entities_image.clone(),
            atlas: TextureAtlas {
                layout: game_resource.assets.entities_texture_atlas.clone(),
                index: config::EntityTile::MarioSmallIdle as usize,
            },
            transform: Transform::from_xyz(init_position.x, init_position.y, 1.0),
            ..default()
        },
        Name::new("mario"),
        Player,
        Intention {
            direction: Direction::Idle,
            jump: false,
        },
        Physics {
            position: init_position,
            is_ground: true,
            mass: 1.0,
            ..Default::default()
        },
    ));
}

fn update_net_force(mut query: Query<(&mut Physics, &Intention)>) {
    let mut net_force = Vec2::new(0.0, 0.0);
    for (mut physics, intention) in &mut query {
        let velocity = physics.velocity;
        if physics.is_ground {
            match intention.direction {
                Direction::Idle => net_force += Vec2::ZERO,
                Direction::Left => net_force += Vec2::new(-60.0, 0.0),
                Direction::Right => net_force += Vec2::new(60.0, 0.0),
            }
        }
        if intention.jump && physics.is_ground {
            net_force += Vec2::new(0.0, 120.0);
        }
        physics.force = net_force
    }
}

fn update_acceleration(mut last_time: Local<f32>, time: Res<Time>, mut query: Query<&mut Physics>) {
    let dt = time.elapsed_seconds() - *last_time;
    for mut physics in &mut query {
        let force = physics.force;
        let mass = physics.mass;
        physics.acceleration += force / mass * dt
    }
    *last_time = time.elapsed_seconds();
}

fn update_velocity(mut last_time: Local<f32>, time: Res<Time>, mut query: Query<&mut Physics>) {
    let dt = time.elapsed_seconds() - *last_time;
    for mut physics in &mut query {
        let acceleration = physics.acceleration;
        physics.velocity += acceleration * dt;
    }
    *last_time = time.elapsed_seconds();
}

fn update_position(mut last_time: Local<f32>, time: Res<Time>, mut query: Query<&mut Physics>) {
    let dt = time.elapsed_seconds() - *last_time;
    for mut physics in &mut query {
        let velocity = physics.velocity;
        physics.position += velocity * dt;
    }
    *last_time = time.elapsed_seconds();
}

fn sync_physics(mut query: Query<(&mut Physics, &mut Transform)>) {
    for (physics, mut transform) in &mut query {
        transform.translation = Vec3::new(physics.position.x, physics.position.y, 1.0);
    }
}

fn mario_controller(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut action_query: Query<&mut Intention, With<Player>>,
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
