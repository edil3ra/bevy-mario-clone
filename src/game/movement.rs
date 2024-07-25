use bevy::prelude::*;

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        record_movement_controller.in_set(AppSet::RecordInput),
    );

    // Apply movement based on controls.
    app.register_type::<Physics>();
    app.add_systems(Update, (apply_movement).chain().in_set(AppSet::Update));
}

#[derive(Reflect, Debug)]
pub enum Direction {
    Idle,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    direction: Direction,
    jump: bool,
}
impl MovementController {
    fn reset(&mut self) {
        self.direction = Direction::default();
        self.jump = false;
    }
}

#[derive(Reflect, Default, Debug, Component)]
#[reflect(Component)]
pub struct Physics {
    pub velocity: Vec3,
    // is_ground: bool,
}

fn record_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController>,
) {
    for mut controller in &mut controller_query {
        controller.reset();
        if input.pressed(KeyCode::ArrowLeft) && input.pressed(KeyCode::ArrowRight) {
            controller.direction = Direction::Idle;
        } else if input.pressed(KeyCode::ArrowLeft) {
            controller.direction = Direction::Left;
        } else if input.pressed(KeyCode::ArrowRight) {
            controller.direction = Direction::Right;
        }
        if input.pressed(KeyCode::Space) {
            controller.jump = true
        }
    }
}

fn apply_movement(time: Res<Time>, mut query: Query<(&mut Physics, &MovementController)>) {
    let dt = time.delta().as_secs_f32();
    for (mut physics, intention) in &mut query {
        let abs_x = physics.velocity.x.abs();
        let mut distance = 0.0; // will be used later
        let direction = match intention.direction {
            Direction::Idle => 0,
            Direction::Left => -1,
            Direction::Right => 1,
        };

        if direction == 0 {
            if physics.velocity.x != 0.0 {
                let decel = abs_x.min(300.0 * dt);
                if physics.velocity.x > 0.0 {
                    physics.velocity.x -= decel;
                } else {
                    physics.velocity.x += decel;
                }
            } else {
                distance = 0.0;
            }
        } else {
            physics.velocity.x += 400.0 * direction as f32 * dt;
        }
        let drag = 1.0 / 5000.0 * physics.velocity.x * abs_x;
        physics.velocity.x -= drag;
        distance = abs_x * dt;
    }
}
