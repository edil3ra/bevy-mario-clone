use bevy::prelude::*;

use super::TileCollision;

#[derive(Default, Resource)]
pub struct TileCollisions(pub Vec<TileCollision>);
