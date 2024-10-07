use bevy::prelude::*;

use crate::game::{
    movement::MovementController,
    physics::{Forces, Vel},
};

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Jump {
    pub ready: f32,
    pub duration: f32,
    pub engage_time: f32,
    pub request_time: f32,
    pub grace_period: f32,
    pub speed_boost: f32,
    pub velocity: f32,
}
