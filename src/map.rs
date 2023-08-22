use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_ecs_tilemap::{
    prelude::{
        get_tilemap_center_transform, TilemapId, TilemapSize, TilemapTexture, TilemapTileSize,
        TilemapType,
    },
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle, TilemapPlugin,
};

use crate::{config, level, AppState, Game};

#[derive(Component)]
struct Index(usize, usize);

#[derive(Debug, Default, Resource)]
// pub struct Map {
//     pub tiles: Vec<Entity>,
//     pub total_row: usize,
//     pub total_column: usize,
//     pub width: usize,
//     pub height: usize,
// }

// impl Map {
//     fn get_index_from_x_y(&self, x: f32, y: f32) -> &Entity {
//         let row = (y / config::TILE_SIZE) as usize * self.total_column ;
//         let col = (x / config::TILE_SIZE) as usize;
//         let index = row + col;
//         &self.tiles[index]
//     }
// }

pub struct MapPlugins;
impl PluginGroup for MapPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(TilemapPlugin)
            .add(MapPlugin {})
    }
}

pub struct MapPlugin {}
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), build_map);
    }
}

// 15,
// 13,

fn build_map(mut commands: Commands, game_resource: Res<Game>) {
    let current_level = level::LevelFile::new(config::LEVELS[game_resource.level.current]);
    let texture_handle = game_resource.assets.tiles_image.clone();

    let map_size = TilemapSize {
        x: current_level.n_columns as u32,
        y: current_level.n_rows as u32,
    };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for y in 0..map_size.y {
        for x in 0..map_size.x {
            let tile_pos = TilePos {
                x,
                y: map_size.y - y - 1,
            };
            let level_char = current_level.get(y as usize, x as usize);
            let texture_index = game_resource
                .map_char_to_texture_index
                .get(&level_char)
                .unwrap();

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(*texture_index as u32),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        size: map_size,
        tile_size,
        grid_size,
        map_type,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),

        // transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}
