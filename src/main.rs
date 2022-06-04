mod level;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use std::collections::HashMap;

pub const LEVEL_COUNT: usize = 1;
const LEVELS: [&str; LEVEL_COUNT] = [include_str!("levels/level0.txt")];
struct TilesHandle(Handle<TextureAtlas>);

#[derive(Component)]
struct Index(i32, i32);
#[derive(Component)]
struct Name(String);

#[derive(Debug)]
struct Game {
    level: level::Level,
    map_to_index: HashMap<char, usize>,
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
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
        .add_startup_system_to_stage(StartupStage::Startup, build_map)
        .run();
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/tiles.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 15, 13);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(TilesHandle(texture_atlas_handle));
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn build_map(mut commands: Commands, game_resource: Res<Game>, tiles_handle: Res<TilesHandle>) {
    let current_level = level::LevelFile::new(LEVELS[game_resource.level.current]);
    for y in 0..current_level.dims.1 {
        for x in 0..current_level.dims.0 {
            let pos = (x, y);
            let tile = current_level.get(pos);
            let index_map = game_resource.map_to_index.get(&tile).unwrap();
            commands.spawn_bundle(SpriteSheetBundle {
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
