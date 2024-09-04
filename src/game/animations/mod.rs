pub mod goomba_animation;
pub mod player_animation;

use std::time::Duration;

use crate::AppSet;
use bevy::prelude::*;

use self::goomba_animation::GoombaAnimation;
use self::player_animation::PlayerAnimation;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_animation_timer::<PlayerAnimation>.in_set(AppSet::TickTimers),
            update_animation_timer::<GoombaAnimation>.in_set(AppSet::TickTimers),
            (update_animation_atlas::<PlayerAnimation>,)
                .chain()
                .in_set(AppSet::Update),
            (update_animation_atlas::<GoombaAnimation>,)
                .chain()
                .in_set(AppSet::Update),
        ),
    );
}

pub trait Animate {
    fn changed(&self) -> bool;
    fn get_atlas_index(&self) -> usize;
    fn update_timer(&mut self, delta: Duration);
}

fn update_animation_timer<T: Animate + Component>(time: Res<Time>, mut query: Query<&mut T>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

fn update_animation_atlas<T: Animate + Component>(mut query: Query<(&T, &mut TextureAtlas)>) {
    for (animation, mut atlas) in &mut query {
        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}
