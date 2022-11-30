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

#[derive(Debug, Resource)]
struct TilesHandle(Handle<TextureAtlas>);

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
                width: 1920.,
                height: 1080.,
                mode: WindowMode::Windowed,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
        .add_startup_system_to_stage(StartupStage::Startup, build_map)
        .add_system_to_stage(CoreStage::PreUpdate, toggle_fullscreen)
        .run();
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/tiles.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 15, 13, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(TilesHandle(texture_atlas_handle));
    let scale_factor = WINDOW_HEIGHT / TILE_SIZE / TILE_MAX_HEIGHT;
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(-TILE_SIZE / 2.0, -TILE_SIZE / 2.0, 1.0),
            scale: Vec3::new(1.0/scale_factor, 1.0/scale_factor, 1.0),
            ..Default::default()
        },
        projection: OrthographicProjection{
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
                    transform: Transform::from_xyz((x * 16) as f32, (y * 16) as f32, 1.0),
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
