use bevy::{
    app::{App, Plugin, PluginGroup, PluginGroupBuilder},
    core::Name,
    ecs::system::{Commands, Res},
    hierarchy::BuildChildren,
    prelude::*,
    transform::components::Transform,
};
use bevy_ecs_tilemap::prelude::*;

use crate::{config, level, AppState, Game, Level};

#[derive(Debug, Default)]
pub struct TileFactory {
    pub name: String,
    pub tileType: TileType,
    pub hasObject: Option<String>,
    pub changable: Option<TileChangable>,
    pub collide: bool,
}

#[derive(Debug, Default)]
pub struct TileChangable {
    toName: String,
    counter: Option<u32>,
}

#[derive(Debug)]
pub enum TileType {
    Fixed { texture_index: u32 },
    Animated { texture_indexes: Vec<u32> },
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Fixed { texture_index: (0) }
    }
}

pub struct MapPlugin {}
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), build_map);
    }
}

pub struct MapPlugins;
impl PluginGroup for MapPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(TilemapPlugin)
            .add(MapPlugin {})
    }
}

fn build_map(mut commands: Commands, game_resource: Res<Game>, levels: Res<Assets<Level>>) {
    // let current_level = level::LevelFile::new(config::LEVELS[game_resource.current_level.current]);
    let current_level = levels
        .get(level.get(game_resource.current_level).unwrap())
        .unwrap();

    // current_level.sprite_sheet
    // let texture_handle = game_resource.assets.texture_entities.clone();
    let texture_handle = game_resource.assets.textures.get("tiles").unwrap().clone();

    // let map_size = TilemapSize {
    //     x: current_level.n_columns as u32,
    //     y: current_level.n_rows as u32,
    // };
    // let tilemap_entity = commands.spawn_empty().id();
    // let mut tile_storage = TileStorage::empty(map_size);

    // let map = commands
    //     .spawn_empty()
    //     .insert((
    //         SpatialBundle {
    //             transform: Transform::from_xyz(0.0, 0., 0.),
    //             ..Default::default()
    //         },
    //         Name::new("map"),
    //     ))
    //     .id();

    // for y in 0..map_size.y {
    //     for x in 0..map_size.x {
    //         let tile_pos = TilePos {
    //             x,
    //             y: map_size.y - y - 1,
    //         };
    //         let level_char = current_level.get(y as usize, x as usize);
    //         let tile_template = game_resource
    //             .map_char_to_texture_index
    //             .get(&level_char)
    //             .unwrap();

    //         let texture_index = match &tile_template.tileType {
    //             TileType::Fixed { texture_index } => *texture_index,
    //             TileType::Animated { texture_indexes } => texture_indexes[0],
    //         };

    //         let tile_entity = commands
    //             .spawn((
    //                 TileBundle {
    //                     position: tile_pos,
    //                     tilemap_id: TilemapId(tilemap_entity),
    //                     texture_index: TileTextureIndex(texture_index),
    //                     ..Default::default()
    //                 },
    //                 Name::new(tile_template.name.clone()),
    //             ))
    //             .id();
    //         commands.entity(map).add_child(tile_entity);
    //         tile_storage.set(&tile_pos, tile_entity);
    //     }
    // }
    // let tile_size = TilemapTileSize {
    //     x: config::TILE_SIZE,
    //     y: config::TILE_SIZE,
    // };
    // let grid_size = tile_size.into();
    // let map_type = TilemapType::default();

    // commands.entity(tilemap_entity).insert(TilemapBundle {
    //     size: map_size,
    //     tile_size,
    //     grid_size,
    //     map_type,
    //     storage: tile_storage,
    //     texture: TilemapTexture::Single(texture_handle),

    //     // transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
    //     ..Default::default()
    // });
}
