pub mod go;
pub mod jump;
pub mod solid;

use bevy::prelude::*;

use self::go::Go;

use super::{physics::PhysicsStep, tiles::update_tile_collisions_resource};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Go>();
    app.add_systems(FixedUpdate, go::update.in_set(PhysicsStep::PreIntegrate));
    app.add_systems(
        FixedUpdate,
        solid::obstruct_tile_collision
            .in_set(PhysicsStep::PostSolvePositions)
            .after(update_tile_collisions_resource),
    );
}
