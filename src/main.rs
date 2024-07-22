mod assets;
mod config;
mod dev_tools;
mod map;
use bevy::asset::Asset;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;

use bevy::render::render_resource::Texture;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_egui::egui::TextBuffer;
use map::{TileFactory, TileType};
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

#[derive(Debug, Default, Resource)]
pub struct Game {
    current_level: String,
    is_fullscreen: bool,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Playing,
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

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
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
        );

        app.add_plugins((assets::plugin, map::plugin));

        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);

        app.init_state::<AppState>();
        app.insert_resource(Game {
            current_level: "1-1".to_string(),
            ..Default::default()
        });
        
        app.add_systems(Startup, setup)
            .add_systems(
                OnEnter(AppState::Playing),
                (
                    spawn_camera, // spawn_mario
                ),
            )
            .add_systems(
                Update,
                (sync_player_action_with_input, update_player)
                    .run_if(in_state(AppState::Playing)),
            );
    }
}



fn setup(mut game_res: ResMut<Game>, mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Playing);
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

// fn update_physics(
//     mut last_time: Local<f32>,
//     time: Res<Time>,
//     mut query: Query<(&mut Physics, &mut Transform)>,
//     mut game_res: ResMut<Game>,
//     levels: Res<Assets<Level>>,
// ) {
//     let t = levels.iter();
//     let dt = time.elapsed_seconds() - *last_time;
//     for (physics, mut transform) in &mut query {
//         transform.translation = transform.translation.add(physics.velocity * dt);
//     }
//     *last_time = time.elapsed_seconds();
// }

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



fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
