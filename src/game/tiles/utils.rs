use bevy::math::Vec2;
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapSize},
    tiles::TilePos,
};

use crate::game::physics::BoxCollider;

pub fn get_tile_x_side_coords(
    prev_pos: Vec2,
    current_pos: Vec2,
    box_: &BoxCollider,
) -> Option<[Vec2; 2]> {
    if prev_pos.x > current_pos.x {
        let left_up = Vec2::new(current_pos.x, current_pos.y + box_.size.y);
        let left_down = Vec2::new(current_pos.x, current_pos.y);
        Some([left_up, left_down])
    } else if prev_pos.x < current_pos.x {
        let right_up = Vec2::new(current_pos.x + box_.size.x, current_pos.y + box_.size.y);
        let right_down = Vec2::new(current_pos.x + box_.size.x, current_pos.y);
        Some([right_up, right_down])
    } else {
        None
    }
}

pub fn get_tile_y_side_coords(
    prev_pos: Vec2,
    current_pos: Vec2,
    box_: &BoxCollider,
) -> Option<[Vec2; 2]> {
    if prev_pos.y > current_pos.y {
        let left_down = Vec2::new(current_pos.x, current_pos.y);
        let right_down = Vec2::new(current_pos.x + box_.size.x, current_pos.y);
        Some([left_down, right_down])
    } else if prev_pos.y < current_pos.y {
        let left_up = Vec2::new(current_pos.x, current_pos.y + box_.size.y);
        let right_up = Vec2::new(current_pos.x + box_.size.x, current_pos.y + box_.size.y);
        Some([left_up, right_up])
    } else {
        None
    }
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
