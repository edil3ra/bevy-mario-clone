mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use std::time::Duration;

use crate::config::GRAVITY;
pub use crate::game::physics::{components::*, resources::*, systems::*};

pub const DT: f32 = 1. / 60.;
const COLLISION_PAIR_VEL_MARGIN_FACTOR: f32 = 2. * DT;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PhysicsStep {
    UpdateAABB,
    CollectCollisionPairs,
    PreIntegrate,
    Integrate,
    SolvePositions,
    PostSolvePositions,
    UpdateVelocities,
    SolveVelocities,
    SyncTransform,
    Debug,
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Pos>()
        .register_type::<PrevPos>()
        .register_type::<Mass>()
        .register_type::<PreSolveVel>()
        .register_type::<Vel>()
        .register_type::<Restitution>()
        .register_type::<Forces>()
        .register_type::<Aabb>()
        .register_type::<BoxCollider>();

    app.configure_sets(
        FixedUpdate,
        (
            PhysicsStep::UpdateAABB,
            PhysicsStep::CollectCollisionPairs,
            PhysicsStep::PreIntegrate,
            PhysicsStep::Integrate,
            PhysicsStep::SolvePositions,
            PhysicsStep::PostSolvePositions,
            PhysicsStep::UpdateVelocities,
            PhysicsStep::SolveVelocities,
            PhysicsStep::SyncTransform,
            PhysicsStep::Debug,
        )
            .chain(),
    );
    app.init_resource::<Contacts>();
    app.init_resource::<CollisionPairs>();
    app.insert_resource(Gravity(Vec2::new(0., -GRAVITY)));
    app.insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f32(DT)));

    app.add_systems(
        FixedUpdate,
        (update_aabb_box).in_set(PhysicsStep::UpdateAABB),
    );
    app.add_systems(
        FixedUpdate,
        (collect_collision_pairs).in_set(PhysicsStep::CollectCollisionPairs),
    );
    app.add_systems(FixedUpdate, (integrate).in_set(PhysicsStep::Integrate));
    app.add_systems(FixedUpdate, (solve_pos).in_set(PhysicsStep::SolvePositions));
    app.add_systems(
        FixedUpdate,
        (update_vel).in_set(PhysicsStep::UpdateVelocities),
    );
    app.add_systems(
        FixedUpdate,
        (solve_vel).in_set(PhysicsStep::SolveVelocities),
    );
    app.add_systems(
        FixedUpdate,
        (sync_transforms).in_set(PhysicsStep::SyncTransform),
    );
    app.add_systems(
        FixedUpdate,
        (draw_box_collider, draw_velocity).in_set(PhysicsStep::Debug),
    );
}
