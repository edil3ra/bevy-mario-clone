use bevy::prelude::*;

use crate::AppSet;

use super::entities::Player;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        record_movement_controller.in_set(AppSet::RecordInput),
    );

    app.add_systems(Update, camera_follow_player.in_set(AppSet::Update));
}

#[derive(Reflect, Debug, Clone)]
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

impl From<ControllerDirection> for i32 {
    fn from(value: ControllerDirection) -> Self {
        match value {
            ControllerDirection::Idle => 0,
            ControllerDirection::Left => -1,
            ControllerDirection::Right => 1,
        }
    }
}

pub enum Direction {
    Right,
    TopRight,
    Top,
    TopLeft,
    Left,
    BottomLeft,
    Bottom,
    BottomRight,
    Neutral,
}

impl From<Vec2> for Direction {
    fn from(value: Vec2) -> Self {
        if value.x > 0. && value.y > 0. {
            Self::TopRight
        } else if value.x < 0. && value.y > 0. {
            Self::TopLeft
        } else if value.x < 0. && value.y < 0. {
            Self::BottomLeft
        } else if value.x > 0. && value.y < 0. {
            Self::BottomRight
        } else if value.x > 0. {
            Self::Right
        } else if value.x < 0. {
            Self::Left
        } else if value.y > 0. {
            Self::Top
        } else if value.y < 0. {
            Self::Bottom
        } else {
            Self::Neutral
        }
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

fn camera_follow_player(
    mut camera: Query<&mut Transform, With<Camera>>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    for mut transform in &mut camera {
        for player_transform in &player {
            transform.translation.x = player_transform.translation.x;
            transform.translation.y = player_transform.translation.y;
        }
    }
}
