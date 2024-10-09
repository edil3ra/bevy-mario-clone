use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapSize},
    tiles::{TileStorage, TileTextureIndex},
};

use crate::game::physics::{Aabb, BoxCollider, Pos, PrevPos};

use super::{components::*, resources::*, utils::*};

pub fn update_animation_tile_timer(time: Res<Time>, mut query: Query<&mut AnimationTile>) {
    for mut animation in &mut query {
        animation.timer.tick(time.delta());
        if animation.timer.finished() {
            animation.frame = (animation.frame + 1) % animation.frames.len();
        }
    }
}

pub fn update_animation_time_atlas(mut query: Query<(&AnimationTile, &mut TileTextureIndex)>) {
    for (animation, mut texture_index) in &mut query {
        if animation.timer.finished() {
            texture_index.0 = animation.frames[animation.frame];
        }
    }
}

pub fn update_tile_collisions_resource(
    mut tile_collisions: ResMut<TileCollisions>,
    tilemap_q: Query<(&TilemapSize, &TilemapGridSize, &TileStorage)>,
    box_q: Query<(Entity, &Pos, &PrevPos, &BoxCollider)>,
) {
    let (map_size, grid_size, tile_storage) = tilemap_q.get_single().unwrap();

    tile_collisions.0.clear();

    for (box_entity, current_pos, prev_pos, box_) in box_q.iter() {
        let mut pos = prev_pos.0;
        pos.y = current_pos.0.y;

        let y_sides = get_tile_y_side_coords(prev_pos.0, pos, box_).map(|tile_coords| {
            tile_coords
                .into_iter()
                .filter_map(|tile_coord| from_world_pos(&tile_coord, map_size, grid_size))
                .filter_map(|tile_pos| {
                    tile_storage
                        .get(&tile_pos)
                        .map(|entity| (entity, Aabb::from(tile_pos)))
                })
                .filter_map(|(entity, tile_aabb)| {
                    let prev_aabb = Aabb::from_vec_size(prev_pos.0, box_.size);
                    let current_aabb = Aabb::from_vec_size(pos, box_.size);
                    if prev_aabb.top() <= tile_aabb.bottom()
                        && current_aabb.top() > tile_aabb.bottom()
                    {
                        Some((entity, YSide::Bottom))
                    } else if prev_aabb.bottom() >= tile_aabb.top()
                        && current_aabb.bottom() < tile_aabb.top()
                    {
                        Some((entity, YSide::Top))
                    } else {
                        None
                    }
                })
                .map(|y_side| TileCollision {
                    from: box_entity,
                    to: y_side.0,
                    x_side: None,
                    y_side: Some(y_side.1),
                })
                .collect::<Vec<_>>()
        });

        if let Some(mut y_sides) = y_sides {
            tile_collisions.0.append(&mut y_sides)
        }

        let mut pos = prev_pos.0;
        pos.x = current_pos.0.x;

        let x_sides = get_tile_x_side_coords(prev_pos.0, pos, box_).map(|tiles_coord| {
            tiles_coord
                .into_iter()
                .filter_map(|tile_coord| from_world_pos(&tile_coord, map_size, grid_size))
                .filter_map(|tile_pos| {
                    tile_storage
                        .get(&tile_pos)
                        .map(|entity| (entity, Aabb::from(tile_pos)))
                })
                .filter_map(|(entity, tile_aabb)| {
                    let prev_aabb = Aabb::from_vec_size(prev_pos.0, box_.size);
                    let current_aabb = Aabb::from_vec_size(pos, box_.size);
                    if prev_aabb.right() <= tile_aabb.left()
                        && current_aabb.right() > tile_aabb.left()
                    {
                        Some((entity, XSide::Left))
                    } else if prev_aabb.left() >= tile_aabb.right()
                        && current_aabb.left() < tile_aabb.right()
                    {
                        Some((entity, XSide::Right))
                    } else {
                        None
                    }
                })
                .map(|x_side| TileCollision {
                    from: box_entity,
                    to: x_side.0,
                    x_side: Some(x_side.1),
                    y_side: None,
                })
                .collect::<Vec<_>>()
        });

        if let Some(mut x_sides) = x_sides {
            tile_collisions.0.append(&mut x_sides)
        }
    }
}
