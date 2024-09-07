pub mod components;
pub mod systems;
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TileTextureIndex;

use crate::AppSet;

use self::components::{AnimationTile, Behaviour, TileName};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TileName>();
    app.register_type::<AnimationTile>();
    app.register_type::<Behaviour>();

    app.add_systems(
        Update,
        (
            update_animation_tile_timer.in_set(AppSet::TickTimers),
            (update_animation_time_atlas,)
                .chain()
                .in_set(AppSet::Update),
        ),
    );
}

fn update_animation_tile_timer(time: Res<Time>, mut query: Query<&mut AnimationTile>) {
    for mut animation in &mut query {
        animation.timer.tick(time.delta());
        if animation.timer.finished() {
            animation.frame = (animation.frame + 1) % animation.frames.len();
        }
    }
}

fn update_animation_time_atlas(mut query: Query<(&AnimationTile, &mut TileTextureIndex)> ) {
    for (animation, mut texture_index) in &mut query {
        if animation.timer.finished() {
            texture_index.0 = animation.frames[animation.frame];
        }
    }
}
