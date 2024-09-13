mod components;
mod resources;
mod systems;
mod utils;

use std::time::Duration;

use crate::config::GRAVITY;
pub use crate::game::physics::{components::*, resources::*, systems::*, utils::*};
use bevy::prelude::*;

pub const DELTA_TIME: f32 = 1. / 60.;
const COLLISION_PAIR_VEL_MARGIN_FACTOR: f32 = 2. * DELTA_TIME;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PhysicsStep {
    UpdateAABB,
    CollectCollisionPairs,
    Integrate,
    SolvePositions,
    PostSolvePositions,
    UpdateVelocities,
    SolveVelocities,
    SyncTransform,
}

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(
        FixedUpdate,
        (
            PhysicsStep::UpdateAABB,
            PhysicsStep::CollectCollisionPairs,
            PhysicsStep::Integrate,
            PhysicsStep::SolvePositions,
            PhysicsStep::PostSolvePositions,
            PhysicsStep::UpdateVelocities,
            PhysicsStep::SolveVelocities,
            PhysicsStep::SyncTransform,
        )
            .chain(),
    );
    app.init_resource::<Contacts>();
    app.init_resource::<CollisionPairs>();
    app.insert_resource(Gravity(Vec2::new(0., -GRAVITY)));
    app.insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f32(
        DELTA_TIME,
    )));

    app.add_systems(FixedUpdate, (update_aabb_box).in_set(PhysicsStep::UpdateAABB));
    app.add_systems(
        FixedUpdate,
        (collect_collision_pairs).in_set(PhysicsStep::CollectCollisionPairs),
    );
    app.add_systems(FixedUpdate, (integrate).in_set(PhysicsStep::Integrate));
    app.add_systems(FixedUpdate, (solve_pos).in_set(PhysicsStep::SolvePositions));
    app.add_systems(FixedUpdate, (update_vel).in_set(PhysicsStep::UpdateVelocities));
    app.add_systems(FixedUpdate, (solve_vel).in_set(PhysicsStep::SolveVelocities));
    app.add_systems(FixedUpdate, (sync_transforms).in_set(PhysicsStep::SyncTransform));
}
