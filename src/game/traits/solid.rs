use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::game::{
    physics::{Aabb, BoxCollider, Pos},
    tiles::{components::Behaviour, TileCollisions, XSide, YSide},
};

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Obstruct(pub bool);

pub fn obstruct(
    tile_collisions: Res<TileCollisions>,
    mut box_q: Query<(&BoxCollider, &mut Pos, &mut Obstruct)>,
    tilemap_q: Query<(&TilePos, &Behaviour)>,
) {
    for tile_collision in &tile_collisions.0 {
        if let Ok((box_, mut pos, mut obstruct)) = box_q.get_mut(tile_collision.from) {
            let (tile_pos, tile_behaviour) = tilemap_q.get(tile_collision.to).unwrap();
            let tile_aabb = Aabb::from(*tile_pos);

            obstruct.0 = false;
            if tile_behaviour.is_solid() {
                if let Some(x_side) = tile_collision.x_side {
                    match x_side {
                        XSide::Left => {
                            pos.0.x = tile_aabb.left() - box_.size.x;
                        }
                        XSide::Right => {
                            pos.0.x = tile_aabb.right();
                        }
                    }
                }
                if let Some(y_side) = tile_collision.y_side {
                    match y_side {
                        YSide::Top => {
                            pos.0.y = tile_aabb.top();
                        }
                        YSide::Bottom => {
                            pos.0.y = tile_aabb.bottom() + box_.size.y;
                        }
                    }
                }
                if tile_collision.x_side.is_some() || tile_collision.y_side.is_some() {
                    obstruct.0 = true;
                }
            }
        }
    }
}
