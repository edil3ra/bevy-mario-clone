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

#[derive(Default, Resource)]
pub struct TileCollisions(pub Vec<TileCollision>);

#[derive(Debug, Clone)]
pub struct TileCollision {
    pub from: Entity,
    pub to: Entity,
    pub x_side: Option<XSide>,
    pub y_side: Option<YSide>,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum XSide {
    #[default]
    Left,
    Right,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum YSide {
    #[default]
    Top,
    Bottom,
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TileName>();
    app.register_type::<AnimationTile>();
    app.register_type::<Behaviour>();

    app.init_resource::<TileCollisions>();

    app.add_systems(
        FixedUpdate,
        (
            update_animation_tile_timer.in_set(AppSet::TickTimers),
            update_animation_time_atlas.in_set(AppSet::Update),
            update_tile_collisions_resource.in_set(PhysicsStep::PostSolvePositions),
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

pub fn update_tile_collisions_resource(
    mut tile_collisions: ResMut<TileCollisions>,
    tilemap_q: Query<(&TilemapSize, &TilemapGridSize, &TileStorage)>,
    box_q: Query<(Entity, &Pos, &PrevPos, &BoxCollider)>,
) {
    let (map_size, grid_size, tile_storage) = tilemap_q.get_single().unwrap();

    tile_collisions.0.clear();

    for (box_entity, current_pos, prev_pos, box_) in box_q.iter() {
        let mut pos = prev_pos.0;
        pos.x = current_pos.0.x;

        let tiles_coord = get_tile_corner_coords(pos, box_)
            .filter_map(|x| from_world_pos(&x, map_size, grid_size));

        let x_sides = tiles_coord
            .filter_map(|tile_coord| {
                if let Some(tile_entity) = tile_storage.get(&tile_coord) {
                    let tile_aabb = Aabb::from(tile_coord);
                    let prev_aabb = Aabb::from_vec_size(prev_pos.0, box_.size);
                    let current_aabb = Aabb::from_vec_size(current_pos.0, box_.size);
                    if prev_aabb.right() <= tile_aabb.left()
                        && current_aabb.right() > tile_aabb.left()
                    {
                        return Some((tile_entity, XSide::Left));
                    } else if prev_aabb.left() >= tile_aabb.right()
                        && current_aabb.left() < tile_aabb.right()
                    {
                        return Some((tile_entity, XSide::Right));
                    } else {
                        return None;
                    }
                }
                None
            })
            .collect::<Vec<_>>();

        let mut pos = prev_pos.0;
        pos.y = current_pos.0.y;

        let tiles_coord = get_tile_corner_coords(pos, box_)
            .filter_map(|x| from_world_pos(&x, map_size, grid_size));

        let y_sides = tiles_coord
            .filter_map(|tile_coord| {
                if let Some(tile_entity) = tile_storage.get(&tile_coord) {
                    let tile_aabb = Aabb::from(tile_coord);
                    let prev_aabb = Aabb::from_vec_size(prev_pos.0, box_.size);
                    let current_aabb = Aabb::from_vec_size(current_pos.0, box_.size);
                    if prev_aabb.top() <= tile_aabb.bottom()
                        && current_aabb.top() > tile_aabb.bottom()
                    {
                        return Some((tile_entity, YSide::Bottom));
                    } else if prev_aabb.bottom() >= tile_aabb.top()
                        && current_aabb.bottom() < tile_aabb.top()
                    {
                        return Some((tile_entity, YSide::Top));
                    } else {
                        return None;
                    }
                }
                None
            })
            .collect::<Vec<_>>();

        for x_side in x_sides.into_iter() {
            tile_collisions.0.push(TileCollision {
                from: box_entity,
                to: x_side.0,
                x_side: Some(x_side.1),
                y_side: None,
            })
        }

        for y_side in y_sides.into_iter() {
            tile_collisions.0.push(TileCollision {
                from: box_entity,
                to: y_side.0,
                x_side: None,
                y_side: Some(y_side.1),
            })
        }
    }
}

fn get_tile_corner_coords(current_pos: Vec2, box_: &BoxCollider) -> std::array::IntoIter<Vec2, 4> {
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
