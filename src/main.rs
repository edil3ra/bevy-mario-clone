mod config;
mod level;
mod physics;

use physics::{PhysicsPlugin, ForceFriction, ForceGravity, ForceJump, ForceRun , Velocity};
use bevy::{prelude::*, render::camera::WindowOrigin, time::FixedTimestep};
use bevy_inspector_egui::WorldInspectorPlugin;
use std::collections::HashMap;

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
                width: config::WINDOW_WITDH,
                height: config::WINDOW_HEIGHT,
                mode: WindowMode::Windowed,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(bevy::window::close_on_esc)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
        .add_startup_system_set_to_stage(
            StartupStage::Startup,
            SystemSet::new()
                .with_system(build_map)
                .with_system(spawn_mario),
        )
        .add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new()
                .with_system(toggle_fullscreen)
                .with_system(move_camera)
                .with_system(mario_controller),
        )
        .add_plugin(PhysicsPlugin)
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

    let scale_factor = config::WINDOW_HEIGHT / config::TILE_SIZE / config::TILE_MAX_HEIGHT;
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(-config::TILE_SIZE / 2.0, -config::TILE_SIZE / 2.0, 1.0),
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
    let current_level = level::LevelFile::new(config::LEVELS[game_resource.level.current]);
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
            sprite: TextureAtlasSprite::new(config::EntityTile::MarioSmallIdle as usize),
            ..default()
        },
        Player,
        // ForceGravity(Vec2::new(0.0, -0.001)),
        ForceGravity(Vec2::new(0.0, 0.0)),
        ForceRun(Vec2::new(0.0, 0.0)),
        ForceJump(Vec2::new(0.0, 0.0)),
        Velocity(Vec2::new(0.0, 0.0)),
    ));
}


fn mario_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut ForceRun, &mut ForceJump), With<Player>>,
) {
    let (mut force_run, mut force_jump) = query.get_single_mut().unwrap();
    if keyboard_input.pressed(KeyCode::Left) {
        force_run.0.x += -0.001;
    } else if keyboard_input.pressed(KeyCode::Right) {
        force_run.0.x += 0.001;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        force_jump.0.y += 0.001;
    }
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
    if keyboard_input.pressed(KeyCode::G) {
        transform.translation.x += -10.0;
    }

    if keyboard_input.pressed(KeyCode::C) {
        transform.translation.x += 10.0;
    }

    if keyboard_input.pressed(KeyCode::R) {
        transform.translation.y += 10.0;
    }

    if keyboard_input.pressed(KeyCode::L) {
        transform.translation.y += -10.0;
    }

    if keyboard_input.pressed(KeyCode::Plus) {
        transform.scale *= 0.8;
    }

    if keyboard_input.pressed(KeyCode::Minus) {
        transform.scale *= 1.2;
    }
}
