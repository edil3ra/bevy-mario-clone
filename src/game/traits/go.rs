use bevy::prelude::*;

use crate::game::{
    movement::MovementController,
    physics::{Forces, Vel},
};

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Go {
    pub direction: i32,
    pub heading: i32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub drag_factor: f32,
    pub distance: f32,
}

pub fn update(
    time: Res<Time>,
    mut query: Query<(&mut Go, &MovementController, &Vel, &mut Forces)>,
) {
    let dt = time.delta().as_secs_f32();
    for (mut go, controller, vel, mut forces) in &mut query {
        go.direction = controller.moving.clone().into();
        let mut new_vel_x = 0.;
        let abs_x = vel.0.x.abs();
        go.distance = 0.0;
        if go.direction == 0 {
            if vel.0.x != 0.0 {
                let decel = abs_x.min(go.deceleration);
                if vel.0.x > 0.0 {
                    new_vel_x = -decel;
                } else {
                    new_vel_x = decel;
                }
            } else {
                go.distance = 0.0;
            }
        } else {
            new_vel_x += go.acceleration * go.direction as f32;
        }
        let updated_vel_x = vel.0.x + (new_vel_x * dt);
        let drag = go.drag_factor / dt * updated_vel_x * updated_vel_x.abs();

        forces.0.push(Vec2::new(new_vel_x, 0.));
        forces.0.push(Vec2::new(-drag, 0.));
        go.distance = abs_x * dt;
    }
}
