pub mod components;
pub mod systems;
use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapSize},
    tiles::{TilePos, TileStorage, TileTextureIndex},
};

use crate::AppSet;

use self::components::{AnimationTile, Behaviour, TileName};

use super::physics::{BoxCollider, PhysicsStep, Pos, PrevPos};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TileName>();
    app.register_type::<AnimationTile>();
    app.register_type::<Behaviour>();

    app.add_systems(
        FixedUpdate,
        (
            update_animation_tile_timer.in_set(AppSet::TickTimers),
            (update_animation_time_atlas, entity_collide_with_obstacles)
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

fn entity_collide_with_obstacles(
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
        let tiles_index = tiles_coord.map(|x| tile_storage.get(&x));
        let mut tiles_behaviour = tiles_index.map(|entity| blocks_q.get(entity.unwrap()).unwrap());
        let has_entity_collided = tiles_behaviour.any(|behaviour| behaviour.has_collide());

        if has_entity_collided {
            current_pos.0.x = prev_pos.0.x;
        }

        let mut pos = prev_pos.0;
        pos.y = current_pos.0.y;

        let tiles_coord =
            get_tile_coords(pos, box_).filter_map(|x| from_world_pos(&x, map_size, grid_size));
        let tiles_index = tiles_coord.map(|x| tile_storage.get(&x));
        let mut tiles_behaviour = tiles_index.map(|entity| blocks_q.get(entity.unwrap()).unwrap());
        let has_entity_collided = tiles_behaviour.any(|behaviour| behaviour.has_collide());

        if has_entity_collided {
            current_pos.0.y = prev_pos.0.y
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
