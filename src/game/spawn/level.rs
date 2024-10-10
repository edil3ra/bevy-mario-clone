use bevy::prelude::*;

use super::{
    entities::{spawn_entities, SpawnEntities},
    map::SpawnMap,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
    app.observe(spawn_entities);
}

#[derive(Event, Debug, Default)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    commands.trigger(SpawnMap);
    commands.trigger(SpawnEntities);
}
