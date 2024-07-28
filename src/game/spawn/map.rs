use bevy::prelude::*;
use bevy_ecs_tilemap::{
    helpers::{
        filling::{fill_tilemap, fill_tilemap_rect},
        geometry::get_tilemap_center_transform,
    },
    map::{TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType},
    tiles::{TilePos, TileStorage, TileTextureIndex},
    TilemapBundle, TilemapPlugin,
};

use crate::game::{
    assets::{HandleMap, Level, LevelKey, TextureKey},
    GameState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TilemapPlugin);
    app.observe(spawn_map);
}

#[derive(Event, Debug)]
pub struct SpawnMap;

fn spawn_map(
    _trigger: Trigger<SpawnMap>,
    mut commands: Commands,
    level_handles: Res<HandleMap<LevelKey>>,
    textures_handles: Res<HandleMap<TextureKey>>,
    levels: ResMut<Assets<Level>>,
    gs: Res<GameState>,
) {
    let map_size = TilemapSize { x: 212, y: 15 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    let level = levels
        .get(level_handles[&gs.current_level].clone_weak().id())
        .unwrap();

    for layer in &level.layers {
        for tile in &layer.tiles {
            if let Some(pattern) = &tile.pattern {
                //ignore for now
            }
            if let Some(style) = &tile.style {
                for range in &tile.ranges {
                    match range[..] {
                        [x1, x2, y1, y2] => fill_tilemap_rect(
                            TileTextureIndex(2),
                            TilePos {x: x1 as u32, y: y1 as u32},
                            TilemapSize {x: x2 as u32, y: y2 as u32},
                            TilemapId(tilemap_entity),
                            &mut commands,
                            &mut tile_storage,
                        ),
                        [x1, x2, y1] => {}
                        [x, y] => {}
                        _ => {}
                    }
                }
            }
        }
    }
    let texture_handle: Handle<Image> = textures_handles[&TextureKey::Tiles].clone_weak();

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle.clone()),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}
