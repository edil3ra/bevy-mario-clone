use bevy::prelude::*;

use crate::AppSet;

use super::physics::{Forces, PhysicsStep, Vel};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        record_movement_controller.in_set(AppSet::RecordInput),
    );

    app.add_systems(
        FixedUpdate,
        apply_movement.in_set(PhysicsStep::PreIntegrate),
    );
}

#[derive(Reflect, Debug)]
pub enum ControllerDirection {
    Idle,
    Left,
    Right,
}

impl Default for ControllerDirection {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    pub moving: ControllerDirection,
    pub jumping: bool,
    pub running: bool,
}
impl MovementController {
    fn reset(&mut self) {
        self.moving = ControllerDirection::default();
        self.jumping = false;
    }

    pub fn is_moving(&self) -> bool {
        match self.moving {
            ControllerDirection::Idle => false,
            ControllerDirection::Left => true,
            ControllerDirection::Right => true,
        }
    }
}

fn record_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController>,
) {
    for mut controller in &mut controller_query {
        controller.reset();
        if input.pressed(KeyCode::ArrowLeft) {
            controller.moving = ControllerDirection::Left;
        } else if input.pressed(KeyCode::ArrowRight) {
            controller.moving = ControllerDirection::Right;
        } else {
            controller.moving = ControllerDirection::Idle;
        }
        if input.pressed(KeyCode::Space) {
            controller.jumping = true
        }
        if input.pressed(KeyCode::ShiftLeft) {
            controller.running = true
        }
    }
}

fn apply_movement(time: Res<Time>, mut query: Query<(&MovementController, &Vel, &mut Forces)>) {
    let dt = time.delta().as_secs_f32();
    for (controller, vel, mut forces) in &mut query {
        let mut new_vel_x = 0.;
        let abs_x = vel.0.x.abs();
        let mut distance = 0.0;
        let direction = match controller.moving {
            ControllerDirection::Idle => 0,
            ControllerDirection::Left => -1,
            ControllerDirection::Right => 1,
        };

        if direction == 0 {
            if vel.0.x != 0.0 {
                let decel = abs_x.min(300.0);
                if vel.0.x > 0.0 {
                    new_vel_x = -decel;
                } else {
                    new_vel_x = decel;
                }
            } else {
                distance = 0.0;
            }
        } else {
            new_vel_x += 400.0 * direction as f32;
        }
        let updated_vel_x = vel.0.x + (new_vel_x * dt);
        let drag = ((1.0 / 5000.0 / dt) * updated_vel_x * updated_vel_x.abs());

        forces.0.push(Vec2::new(new_vel_x, 0.));
        forces.0.push(Vec2::new(-drag, 0.));
        distance = abs_x * dt;
    }
}
