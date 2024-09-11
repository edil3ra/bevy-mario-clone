pub mod components;
pub mod systems;
use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage, TileTextureIndex},
};

use crate::AppSet;

use self::components::{AnimationTile, Behaviour, TileName};

use super::physics::{BoxCollider, Pos};

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
                .in_set(AppSet::Update),
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
    mut box_q: Query<(&mut Pos, &BoxCollider)>,
    blocks_q: Query<&Behaviour>,
) {
    let (map_size, grid_size, map_type, tile_storage) = tilemap_q.get_single().unwrap();
    for (mut box_pos, box_) in box_q.iter_mut() {
        let left_up = Vec2::new(
            box_pos.0.x - box_.size.x / 2.,
            box_pos.0.y - box_.size.y / 2.,
        );
        let right_up = Vec2::new(
            box_pos.0.x + box_.size.x / 2.,
            box_pos.0.y - box_.size.y / 2.,
        );
        let left_down = Vec2::new(
            box_pos.0.x - box_.size.x / 2.,
            box_pos.0.y + box_.size.y / 2.,
        );
        let right_down = Vec2::new(
            box_pos.0.x + box_.size.x / 2.,
            box_pos.0.y + box_.size.y / 2.,
        );
        let tile_positions = [left_up, right_up, left_down, right_down]
            .into_iter()
            .filter_map(|x| TilePos::from_world_pos(&x, map_size, grid_size, map_type));

        for tile_pos in tile_positions {
            if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                let behaviour = blocks_q.get(tile_entity).unwrap();
                match behaviour {
                    Behaviour::Ground => {
                        box_pos.0.x = 0.;
                        box_pos.0.y = 100.;
                    }
                    Behaviour::Brick => todo!(),
                    Behaviour::Coin => todo!(),
                    Behaviour::None => {}
                }
            }
        }
    }
}
