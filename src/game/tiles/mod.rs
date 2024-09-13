pub mod components;
pub mod systems;
use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage, TileTextureIndex},
};

use crate::AppSet;

use self::components::{AnimationTile, Behaviour, TileName};

use super::physics::{Aabb, BoxCollider, PhysicsStep, Pos, PrevPos, Vel};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TileName>();
    app.register_type::<AnimationTile>();
    app.register_type::<Behaviour>();

    app.add_systems(
        FixedUpdate,
        (
            update_animation_tile_timer.in_set(AppSet::TickTimers),
            (update_animation_time_atlas, collide)
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

fn collide(
    tilemap_q: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &TileStorage)>,
    mut box_q: Query<(&mut Pos, &mut Vel, &PrevPos,  &BoxCollider)>,
    blocks_q: Query<&Behaviour>,
) {
    let (map_size, grid_size, map_type, tile_storage) = tilemap_q.get_single().unwrap();
    for (mut current_pos, mut vel, prev_pos, box_) in box_q.iter_mut() {
        let left_up = Vec2::new(
            current_pos.0.x - box_.size.x / 2.,
            current_pos.0.y - box_.size.y / 2.,
        );
        let right_up = Vec2::new(
            current_pos.0.x + box_.size.x / 2.,
            current_pos.0.y - box_.size.y / 2.,
        );
        let left_down = Vec2::new(
            current_pos.0.x - box_.size.x / 2.,
            current_pos.0.y + box_.size.y / 2.,
        );
        let right_down = Vec2::new(
            current_pos.0.x + box_.size.x / 2.,
            current_pos.0.y + box_.size.y / 2.,
        );
        let tile_coords = [left_up, right_up, left_down, right_down]
            .into_iter()
            .filter_map(|x| TilePos::from_world_pos(&x, map_size, grid_size, map_type));

        for tile_coord in tile_coords {
            if let Some(tile_entity) = tile_storage.get(&tile_coord) {
                let tile_aabb = Aabb::from(tile_coord);
                let behaviour = blocks_q.get(tile_entity).unwrap();
                let current_aabb = Aabb::new(
                    Vec2::new(
                        current_pos.0.x,
                        current_pos.0.y,
                        
                    ),
                    Vec2::new(
                        current_pos.0.x + box_.size.x,
                        current_pos.0.y + box_.size.y,
                    ),
                );

                let prev_aabb = Aabb::new(
                    Vec2::new(
                        prev_pos.0.x,
                        prev_pos.0.y,
                    ),
                    Vec2::new(
                        prev_pos.0.x + box_.size.x,
                        prev_pos.0.y + box_.size.y,
                    ),
                );

                match behaviour {
                    Behaviour::Ground => {
                        if prev_aabb.top() >= tile_aabb.bottom()
                            && current_aabb.top() < tile_aabb.bottom()
                        {
                            current_pos.0.y = tile_aabb.bottom();
                            vel.0.y = 0.;
                        }

                        if prev_aabb.bottom() >= tile_aabb.top()
                            && current_aabb.bottom() < tile_aabb.top()
                        {
                            current_pos.0.y = tile_aabb.top();
                            vel.0.y = 0.;
                        }

                        if prev_aabb.left() >= tile_aabb.right()
                            && current_aabb.left() < tile_aabb.right()
                        {
                            current_pos.0.x = tile_aabb.right();
                        }
                        
                        if prev_aabb.right() >= tile_aabb.left()
                            && current_aabb.right() < tile_aabb.left()
                        {
                            current_pos.0.x = tile_aabb.right();
                        }

                    }
                    Behaviour::Brick => todo!(),
                    Behaviour::Coin => todo!(),
                    Behaviour::None => {}
                }
            }
        }
    }
}
