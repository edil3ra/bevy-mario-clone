pub mod go;
pub mod jump;
pub mod solid;

use bevy::prelude::*;

use self::go::Go;

use super::{physics::PhysicsStep, tiles::systems::update_tile_collisions_resource};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Go>();
    app.add_systems(FixedPostUpdate, ((go::update, jump::update),).chain());

    app.add_systems(
        FixedUpdate,
        ((solid::obstruct, jump::obstruct),)
            .chain()
            .in_set(PhysicsStep::PostSolvePositions)
            .after(update_tile_collisions_resource),
    );
}
