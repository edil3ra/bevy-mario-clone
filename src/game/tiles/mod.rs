pub mod components;
pub mod systems;
use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapSize},
    tiles::{TilePos, TileStorage, TileTextureIndex},
};

use crate::AppSet;

use self::components::{AnimationTile, Behaviour, TileName};

use super::physics::{Aabb, BoxCollider, PhysicsStep, Pos, PrevPos};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TileName>();
    app.register_type::<AnimationTile>();
    app.register_type::<Behaviour>();

    app.add_systems(
        FixedUpdate,
        (
            update_animation_tile_timer.in_set(AppSet::TickTimers),
            (update_animation_time_atlas, entity_collide_with_tiles)
                .chain()
                .in_set(PhysicsStep::PostSolvePositions),
        ),
    );
}

fn update_animation_tile_timer(time: Res<Time>, mut query: Query<&mut AnimationTile>) {
    for mut animation in &mut query {
        animation.timer.tick(time.delta());
        if animation.timer.finished() {
            animation.frame = (animation.frame + 1) % animation.frames.len();
        }
    }
}

fn update_animation_time_atlas(mut query: Query<(&AnimationTile, &mut TileTextureIndex)>) {
    for (animation, mut texture_index) in &mut query {
        if animation.timer.finished() {
            texture_index.0 = animation.frames[animation.frame];
        }
    }
}

fn entity_collide_with_tiles(
    tilemap_q: Query<(&TilemapSize, &TilemapGridSize, &TileStorage)>,
    mut box_q: Query<(&mut Pos, &PrevPos, &BoxCollider)>,
    blocks_q: Query<&Behaviour>,
) {
    let (map_size, grid_size, tile_storage) = tilemap_q.get_single().unwrap();
    for (mut current_pos, prev_pos, box_) in box_q.iter_mut() {
        let mut pos = prev_pos.0;
        pos.x = current_pos.0.x;

        let tiles_coord =
            get_tile_coords(pos, box_).filter_map(|x| from_world_pos(&x, map_size, grid_size));

        for tile_coord in tiles_coord {
            if let Some(tile_entity) = tile_storage.get(&tile_coord) {
                let tile_aabb = Aabb::from(tile_coord);
                let prev_aabb = Aabb::new(
                    Vec2::new(prev_pos.0.x, prev_pos.0.y),
                    Vec2::new(prev_pos.0.x + box_.size.x, prev_pos.0.y + box_.size.y),
                );
                let current_aabb = Aabb::new(
                    Vec2::new(pos.x, pos.y),
                    Vec2::new(pos.x + box_.size.x, pos.y + box_.size.y),
                );

                let behaviour = blocks_q.get(tile_entity).unwrap();

                if behaviour.is_solid() {
                    if prev_aabb.right() <= tile_aabb.left()
                        && current_aabb.right() > tile_aabb.left()
                    {
                        current_pos.0.x = tile_aabb.left() - box_.size.x;
                    }

                    if prev_aabb.left() >= tile_aabb.right()
                        && current_aabb.left() < tile_aabb.right()
                    {
                        current_pos.0.x = tile_aabb.right();
                    }
                }
            }
        }

        let mut pos = prev_pos.0;
        pos.y = current_pos.0.y;

        let tiles_coord =
            get_tile_coords(pos, box_).filter_map(|x| from_world_pos(&x, map_size, grid_size));

        for tile_coord in tiles_coord {
            if let Some(tile_entity) = tile_storage.get(&tile_coord) {
                let tile_aabb = Aabb::from(tile_coord);
                let prev_aabb = Aabb::new(
                    Vec2::new(prev_pos.0.x, prev_pos.0.y),
                    Vec2::new(prev_pos.0.x + box_.size.x, prev_pos.0.y + box_.size.y),
                );
                let current_aabb = Aabb::new(
                    Vec2::new(pos.x, pos.y),
                    Vec2::new(pos.x + box_.size.x, pos.y + box_.size.y),
                );

                let behaviour = blocks_q.get(tile_entity).unwrap();

                if behaviour.is_solid() {
                    if prev_aabb.top() <= tile_aabb.bottom()
                        && current_aabb.top() > tile_aabb.bottom()
                    {
                        current_pos.0.y = tile_aabb.bottom() + box_.size.y;
                    }
                    if prev_aabb.bottom() >= tile_aabb.top()
                        && current_aabb.bottom() < tile_aabb.top()
                    {
                        current_pos.0.y = tile_aabb.top();
                    }
                }
            }
        }
    }
}

fn get_tile_coords(current_pos: Vec2, box_: &BoxCollider) -> std::array::IntoIter<Vec2, 4> {
    let left_up = Vec2::new(current_pos.x, current_pos.y + box_.size.y);
    let right_up = Vec2::new(current_pos.x + box_.size.x, current_pos.y + box_.size.y);
    let left_down = Vec2::new(current_pos.x, current_pos.y);
    let right_down = Vec2::new(current_pos.x + box_.size.x / 2., current_pos.y);

    [left_up, right_up, left_down, right_down].into_iter()
}

pub fn from_world_pos(
    world_pos: &Vec2,
    map_size: &TilemapSize,
    grid_size: &TilemapGridSize,
) -> Option<TilePos> {
    let x = (world_pos.x / grid_size.x).floor() as i32;
    let y = (world_pos.y / grid_size.y).floor() as i32;
    TilePos::from_i32_pair(x, y, map_size)
}
