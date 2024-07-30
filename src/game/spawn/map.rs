use bevy::prelude::*;
use bevy_ecs_tilemap::{
    helpers::{
        filling::{fill_tilemap, fill_tilemap_rect},
        geometry::get_tilemap_center_transform,
    },
    map::{TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType},
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle, TilemapPlugin,
};

use crate::game::{
    assets::{HandleMap, LevelAsset, LevelKey, PatternAsset, PatternKey, TextureKey},
    tiles::OverWorld,
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
    pattern_handles: Res<HandleMap<PatternKey>>,
    textures_handles: Res<HandleMap<TextureKey>>,
    levels: ResMut<Assets<LevelAsset>>,
    patterns: ResMut<Assets<PatternAsset>>,
    gs: Res<GameState>,
) {
    let texture_handle: Handle<Image> = textures_handles[&TextureKey::Tiles].clone_weak();
    let map_size = TilemapSize { x: 212, y: 15 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    let level = levels
        .get(level_handles[&gs.current_level].clone_weak().id())
        .unwrap();

    for (index, layer) in level.layers.iter().enumerate() {
        let map = commands
            .spawn_empty()
            .insert((
                SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0., 0.),
                    ..Default::default()
                },
                Name::new(format!("layer-{}", index)),
            ))
            .id();

        let pattern_key = PatternKey::from(level.pattern_sheet.as_ref());
        let pattern_handle = pattern_handles.get(&pattern_key).unwrap();
        let patterns = &patterns
            .get(pattern_handle.clone_weak().id())
            .unwrap()
            .patterns;

        for tile in &layer.tiles {
            if let Some(key) = tile.pattern.as_ref() {
                let pattern = patterns.get(key).unwrap();

                for range in &tile.ranges {
                    match range[..] {
                        [x, y] => {
                            for pattern_tile in &pattern.tiles {
                                let style = pattern_tile.style.as_ref();
                                if style.is_none() {
                                    break; // handle recursion later
                                }
                                let texture_index = TileTextureIndex(OverWorld::from(
                                    style.unwrap().as_ref(),
                                )
                                    as u32);
                                for pattern_range in &pattern_tile.ranges {
                                    match pattern_range[..] {
                                        [x_pattern, y_pattern] => {
                                            let tile_pos = TilePos {
                                                x: (x + x_pattern) as u32,
                                                y: 14 - (y + y_pattern) as u32,
                                            };
                                            let tile_entity = commands
                                                .spawn((
                                                    TileBundle {
                                                        position: tile_pos,
                                                        tilemap_id: TilemapId(tilemap_entity),
                                                        texture_index,
                                                        ..Default::default()
                                                    },
                                                    Name::new(format!("x:{}-y:{}", x, y)),
                                                ))
                                                .id();
                                            commands.entity(map).add_child(tile_entity);
                                            tile_storage.set(&tile_pos, tile_entity);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            if let Some(style) = tile.style.as_ref() {
                for range in &tile.ranges {
                    match range[..] {
                        [x1, x2, y1, y2] => {
                            for x in x1..x1 + x2 {
                                for y in y1..y1 + y2 {
                                    let tile_pos = TilePos {
                                        x: x as u32,
                                        y: 14 - y as u32,
                                    };
                                    let texture_index =
                                        TileTextureIndex(OverWorld::from(style.as_ref()) as u32);
                                    let tile_entity = commands
                                        .spawn((
                                            TileBundle {
                                                position: tile_pos,
                                                tilemap_id: TilemapId(tilemap_entity),
                                                texture_index,
                                                ..Default::default()
                                            },
                                            Name::new(format!("x:{}-y:{}", x, y)),
                                        ))
                                        .id();
                                    commands.entity(map).add_child(tile_entity);
                                    tile_storage.set(&tile_pos, tile_entity);
                                }
                            }
                        }
                        [x1, x2, y1] => {}
                        [x, y] => {}
                        _ => {}
                    };
                }
            }
        }
    }

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
        ..Default::default()
    });
}
