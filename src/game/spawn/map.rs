use bevy::{prelude::*, utils::HashMap};
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
    assets::{
        HandleMap, LevelAsset, LevelKey, LevelLayerAsset, LevelTileAsset, PatternAsset, PatternKey,
        PatternTilesAsset, TextureKey,
    },
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
        let map_entity = commands
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

        create_tiles(
            &mut commands,
            &layer.tiles,
            patterns,
            tilemap_entity,
            map_entity,
            &mut tile_storage,
            None,
        );
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

fn create_tiles(
    commands: &mut Commands,
    tiles: &[LevelTileAsset],
    patterns: &HashMap<String, PatternTilesAsset>,
    tilemap_entity: Entity,
    map_entity: Entity,
    tile_storage: &mut TileStorage,
    position: Option<&[i32]>,
) {
    for tile in tiles {
        if let Some(key) = tile.pattern.as_ref() {
            let pattern = patterns.get(key).unwrap();
            for range_position in &tile.ranges {
                let mut new_position: Vec<i32> = Vec::with_capacity(4);
                if let Some(position) = position {
                    new_position.push(
                        *range_position.first().unwrap_or(&0) + *position.first().unwrap_or(&0),
                    );
                    new_position.push(
                        *range_position.get(1).unwrap_or(&0) + *position.get(1).unwrap_or(&0),
                    );
                    new_position.push(
                        *range_position.get(2).unwrap_or(&0) + *position.get(2).unwrap_or(&0),
                    );
                    new_position.push(
                        *range_position.get(3).unwrap_or(&0) + *position.get(3).unwrap_or(&0),
                    );
                }
                let new_position = if new_position.is_empty() {
                    range_position.as_ref()
                } else {
                    new_position.as_ref()
                };

                create_tiles(
                    commands,
                    &pattern.tiles,
                    patterns,
                    tilemap_entity,
                    map_entity,
                    tile_storage,
                    Some(new_position),
                )
            }
        }
        if tile.style.is_some() {
            for range in &tile.ranges {
                match range[..] {
                    [x1, x2, y1, y2] => {
                        for x in x1..x1 + x2 {
                            for y in y1..y1 + y2 {
                                let tile_pos = TilePos {
                                    x: x as u32,
                                    y: 14 - y as u32,
                                };
                                create_tile(
                                    commands,
                                    tile,
                                    tile_pos,
                                    tilemap_entity,
                                    map_entity,
                                    tile_storage,
                                );
                            }
                        }
                    }
                    [x1, x2, y1] => {
                        for x in x1..x1 + x2 {
                            let tile_pos = TilePos {
                                x: x as u32,
                                y: 14 - y1 as u32,
                            };
                            create_tile(
                                commands,
                                tile,
                                tile_pos,
                                tilemap_entity,
                                map_entity,
                                tile_storage,
                            );
                        }
                    }
                    [x, y] => {
                        let mut tile_pos = TilePos {
                            x: x as u32,
                            y: y as u32,
                        };
                        if let Some(position) = position {
                            tile_pos.x += position[0] as u32;
                            tile_pos.y += position[1] as u32;
                        }
                        tile_pos.y = 14 - tile_pos.y;
                        create_tile(
                            commands,
                            tile,
                            tile_pos,
                            tilemap_entity,
                            map_entity,
                            tile_storage,
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn create_tile(
    commands: &mut Commands,
    tile: &LevelTileAsset,
    tile_pos: TilePos,
    tilemap_entity: Entity,
    map_entity: Entity,
    tile_storage: &mut TileStorage,
) {
    let texture_index =
        TileTextureIndex(OverWorld::from(tile.style.as_ref().unwrap().as_ref()) as u32);
    let tile_entity = commands
        .spawn((
            TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index,
                ..Default::default()
            },
            Name::new(format!("x:{}-y:{}", tile_pos.x, tile_pos.y)),
        ))
        .id();
    commands.entity(map_entity).add_child(tile_entity);
    tile_storage.set(&tile_pos, tile_entity);
}
