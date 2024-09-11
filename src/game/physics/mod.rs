mod components;
mod resources;
mod systems;

use std::time::Duration;

pub use crate::game::physics::{components::*, resources::*, systems::*};
use bevy::prelude::*;

pub const DELTA_TIME: f32 = 1. / 60.;
const COLLISION_PAIR_VEL_MARGIN_FACTOR: f32 = 2. * DELTA_TIME;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum Step {
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
            Step::UpdateAABB,
            Step::CollectCollisionPairs,
            Step::Integrate,
            Step::SolvePositions,
            Step::PostSolvePositions,
            Step::UpdateVelocities,
            Step::SolveVelocities,
            Step::SyncTransform,
        )
            .chain(),
    );
    app.init_resource::<Contacts>();
    app.init_resource::<CollisionPairs>();
    app.insert_resource(Gravity(Vec2::new(0., -1500.)));
    app.insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f32(
        DELTA_TIME,
    )));

    app.add_systems(FixedUpdate, (update_aabb_box).in_set(Step::UpdateAABB));
    app.add_systems(
        FixedUpdate,
        (collect_collision_pairs).in_set(Step::CollectCollisionPairs),
    );
    app.add_systems(FixedUpdate, (integrate).in_set(Step::Integrate));
    app.add_systems(FixedUpdate, (solve_pos).in_set(Step::SolvePositions));
    app.add_systems(FixedUpdate, (update_vel).in_set(Step::UpdateVelocities));
    app.add_systems(FixedUpdate, (solve_vel).in_set(Step::SolveVelocities));
    app.add_systems(FixedUpdate, (sync_transforms).in_set(Step::SyncTransform));
}
