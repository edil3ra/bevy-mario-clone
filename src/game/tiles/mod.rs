pub mod components;
pub mod systems;
use bevy::prelude::*;

use self::components::{AnimationTile, Behaviour, TileName};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TileName>();
    app.register_type::<AnimationTile>();
    app.register_type::<Behaviour>();
}
