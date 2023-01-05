mod config;
mod debug;
mod level;
mod map;
mod physics;
use bevy::utils::HashSet;
use bevy::{prelude::*, render::camera::WindowOrigin};
use debug::DebugPlugins;
use map::MapPlugin;
use physics::{Body, Force, ForceKind, PhysicsPlugin};
use std::collections::HashMap;


#[derive(Component)]
struct Player;


#[derive(Debug, Default)]
pub struct AssetsHandle {
    tiles: Handle<TextureAtlas>,
    sprites: Handle<TextureAtlas>
}

#[derive(Debug, Default, Resource)]
pub struct Game {
    assets: AssetsHandle,
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
        .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
        .add_plugins(DebugPlugins)
        .add_plugin(PhysicsPlugin {
            init_gravity: Vec2::new(0., 1.0)
        })
        .add_plugin(MapPlugin {})
        .add_startup_system_set_to_stage(
            StartupStage::Startup,
            SystemSet::new()
                .with_system(spawn_mario),
        )
        .add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new().with_system(mario_controller),
        )
        .run();
}

fn load_assets(
    mut commands: Commands,
    mut game_res: ResMut<Game>,
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

    game_res.assets.tiles = tiles_texture_atlas_handle;
    game_res.assets.sprites = sprites_texture_atlas_handle;

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

fn spawn_mario(
    mut commands: Commands,
    game_resource: Res<Game>,
) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: game_resource.assets.sprites.clone(),
            transform: Transform::from_xyz(32., 32., 1.0),
            sprite: TextureAtlasSprite::new(config::EntityTile::MarioSmallIdle as usize),
            ..default()
        },
        Name::new("mario"),
        Player,
        Body::new(
            1.0,
            Vec2::new(0., 0.),
            HashSet::new(),
            // HashSet::from_iter(vec![Force::new(ForceKind::Gravity, Vec2::new(0.0, 0.1))]),
        ),
    ));
}

fn mario_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Body, With<Player>>,
) {
    let forces = &mut query.get_single_mut().unwrap().forces;
    let mut pressed = false;
    if keyboard_input.pressed(KeyCode::Left) {
        forces.replace(Force::new(ForceKind::Run, Vec2::new(-1.0, 0.0)));
        pressed = true;
    } else if keyboard_input.pressed(KeyCode::Right) {
        forces.replace(Force::new(ForceKind::Run, Vec2::new(0.001, 0.0)));
        pressed = true;
    }

    if !pressed {
        forces.replace(Force::new(ForceKind::Run, Vec2::new(0.0, 0.0)));
    }

    if keyboard_input.pressed(KeyCode::Space) {
        forces.replace(Force::new(ForceKind::Jump, Vec2::new(0.0, 0.1)));
    }
}
