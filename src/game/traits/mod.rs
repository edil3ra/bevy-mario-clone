pub mod go;

use bevy::prelude::*;

use self::go::Go;

use super::physics::PhysicsStep;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Go>();
    app.add_systems(FixedUpdate, go::update.in_set(PhysicsStep::PreIntegrate));
}
