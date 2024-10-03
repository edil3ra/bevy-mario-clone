use bevy::prelude::*;

use crate::game::{
    movement::MovementController,
    physics::{Forces, Vel},
};

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Go {
    pub direction: i32, // [-1, 0, 1]
    pub heading: i32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub drag_factor_x: f32,
    pub distance: f32,
}

pub fn update(
    time: Res<Time>,
    mut query: Query<(&mut Go, &MovementController, &Vel, &mut Forces)>,
) {
    let dt = time.delta().as_secs_f32();
    for (mut go, controller, vel, mut forces) in &mut query {
        go.direction = controller.moving.clone().into();
        let abs_x = vel.0.x.abs();

        if go.direction != 0 {
            let acc = Vec2::new(go.acceleration * go.direction as f32, 0.);
            forces.0.push(acc);
        } else if vel.0.x != 0.0 {
            let decel = Vec2::new(abs_x.min(go.deceleration), 0.);
            if vel.0.x > 0.0 {
                forces.0.push(-decel);
            } else {
                forces.0.push(decel);
            }
        } else {
            go.distance = 0.;
        }
        go.distance += abs_x * dt;
    }
}
