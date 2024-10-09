pub mod components;
pub mod resources;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use super::physics::PhysicsStep;
use crate::AppSet;

use self::components::*;
use self::resources::*;
use self::systems::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TileName>();
    app.register_type::<AnimationTile>();
    app.register_type::<Behaviour>();

    app.init_resource::<TileCollisions>();

    app.add_systems(
        FixedUpdate,
        (
            update_animation_tile_timer.in_set(AppSet::TickTimers),
            update_animation_time_atlas.in_set(AppSet::Update),
            update_tile_collisions_resource.in_set(PhysicsStep::PostSolvePositions),
        ),
    );
}
