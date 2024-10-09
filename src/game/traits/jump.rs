use bevy::prelude::*;

use crate::game::{
    physics::Vel,
    tiles::{
        components::{Behaviour, YSide},
        resources::TileCollisions,
    },
};

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Jump {
    pub ready: i32,
    pub engage_time: f32,
    pub request_time: f32,
    pub duration: f32,
    pub grace_period: f32,
    pub speed_boost: f32,
    pub velocity: f32,
}

impl Jump {
    pub fn is_falling(&self) -> bool {
        self.ready < 0
    }

    pub fn start(&mut self) {
        self.request_time = self.grace_period;
    }

    pub fn cancel(&mut self) {
        self.engage_time = 0.;
        self.request_time = 0.;
    }
}

pub fn obstruct(
    tile_collisions: Res<TileCollisions>,
    mut jump_q: Query<&mut Jump>,
    tilemap_q: Query<&Behaviour>,
) {
    for tile_collision in &tile_collisions.0 {
        if let Ok(mut jump) = jump_q.get_mut(tile_collision.from) {
            let tile_behaviour = tilemap_q.get(tile_collision.to).unwrap();
            if tile_behaviour.is_solid() {
                if let Some(y_side) = tile_collision.y_side {
                    match y_side {
                        YSide::Top => {
                            jump.ready = 1;
                        }
                        YSide::Bottom => {
                            jump.cancel();
                        }
                    }
                }
            }
        }
    }
}

pub fn update(time: Res<Time>, mut query: Query<(&mut Jump, &mut Vel)>) {
    for (mut jump, mut vel) in query.iter_mut() {
        if jump.request_time > 0. {
            if jump.ready > 0 {
                jump.engage_time = jump.duration;
                jump.request_time = 0.;
            }
            jump.request_time -= time.delta_seconds();
        }

        if jump.engage_time > 0. {
            vel.0.y = jump.velocity + vel.0.x.abs() * jump.speed_boost;
            jump.engage_time -= time.delta_seconds();
        }
        jump.ready -= 1;
    }
}
