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

use crate::{
    config::{MAP_HEIGHT, TILE_SIZE},
    game::{
        assets::{
            HandleMap, LevelAsset, LevelKey, LevelTileAsset, PatternAsset, PatternKey,
            PatternTilesAsset, TextureKey,
        },
        tiles::components::{AnimationTile, AnimationTileBuilder, Tile},
        GameState,
    },
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
    let tilemap_entity = commands.spawn(Name::new("TileMap")).id();
    let map_entity = commands
        .spawn((
            Name::new("Map"),
            SpatialBundle {
                ..Default::default()
            },
        ))
        .id();

    let level = levels
        .get(level_handles[&gs.current_level].clone_weak().id())
        .unwrap();

    for (index, layer) in level.layers.iter().enumerate() {
        let layer_entity = commands
            .spawn((
                Name::new(format!("Layer-{}", index + 1)),
                SpatialBundle {
                    ..Default::default()
                },
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
            layer_entity,
            &mut tile_storage,
            None,
        );
        commands
            .get_entity(map_entity)
            .unwrap()
            .add_child(layer_entity);
    }

    let tile_size = TilemapTileSize {
        x: TILE_SIZE as f32,
        y: TILE_SIZE as f32,
    };
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
    layer_entity: Entity,
    tile_storage: &mut TileStorage,
    previous_position: Option<TilePos>,
) {
    for tile in tiles {
        let mut positions: Vec<(TilePos, &LevelTileAsset)> = Vec::with_capacity(0x64);
        for range in &tile.ranges {
            match range[..] {
                [x1, x2, y1, y2] => {
                    for x in x1..x1 + x2 {
                        for y in y1..y1 + y2 {
                            positions.push((
                                TilePos {
                                    x: x as u32,
                                    y: y as u32,
                                },
                                tile,
                            ));
                        }
                    }
                }
                [x1, x2, y] => {
                    for x in x1..x1 + x2 {
                        positions.push((
                            TilePos {
                                x: x as u32,
                                y: y as u32,
                            },
                            tile,
                        ));
                    }
                }
                [x, y] => {
                    positions.push((
                        TilePos {
                            x: x as u32,
                            y: y as u32,
                        },
                        tile,
                    ));
                }
                _ => {}
            }
        }

        for (mut current_position, tile) in positions {
            if let Some(previous_position) = previous_position {
                current_position.x += previous_position.x;
                current_position.y += previous_position.y;
            }
            if tile.style.is_some() {
                current_position.y = MAP_HEIGHT - current_position.y;
                create_tile(
                    commands,
                    tile,
                    current_position,
                    tilemap_entity,
                    layer_entity,
                    tile_storage,
                );
            }

            if let Some(key) = tile.pattern.as_ref() {
                let pattern = patterns.get(key).unwrap();
                create_tiles(
                    commands,
                    &pattern.tiles,
                    patterns,
                    tilemap_entity,
                    layer_entity,
                    tile_storage,
                    Some(current_position),
                )
            }
        }
    }
}

pub fn create_tile(
    commands: &mut Commands,
    tile: &LevelTileAsset,
    tile_pos: TilePos,
    tilemap_entity: Entity,
    layer_entity: Entity,
    tile_storage: &mut TileStorage,
) {
    let tile = Tile::from(tile.style.as_ref().unwrap().as_ref());
    let texture_index = match &tile.animation {
        AnimationTileBuilder::Single(index) => index,
        AnimationTileBuilder::Multiple {
            frames,
            frame_duration: _,
        } => frames.first().unwrap(),
    };

    let tile_entity = commands
        .spawn((
            TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: TileTextureIndex(*texture_index),
                ..Default::default()
            },
            tile.name,
            tile.behaviour,
            Name::new(format!(
                "(name: {}, x: {}, y: {})",
                tile.name, tile_pos.x, tile_pos.y
            )),
        ))
        .id();

    if let AnimationTileBuilder::Multiple {
        frame_duration: frame_len,
        frames,
    } = tile.animation
    {
        commands.entity(tile_entity).insert(AnimationTile {
            frames,
            frame: 0,
            timer: Timer::new(frame_len, TimerMode::Repeating),
        });
    }

    commands.entity(layer_entity).add_child(tile_entity);
    tile_storage.set(&tile_pos, tile_entity);
}
