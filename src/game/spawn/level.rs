use bevy::prelude::*;

// use crate::dev_tools::SpawnTileLabelsDebug;

use crate::game::entities::SpawnEntities;

use super::map::SpawnMap;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    commands.trigger(SpawnMap);
    commands.trigger(SpawnEntities);
}
