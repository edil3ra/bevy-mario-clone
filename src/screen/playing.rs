use bevy::prelude::*;

use super::Screen;
use crate::{game::spawn::level::SpawnLevel, AppSet};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Playing),
        enter_playing.in_set(AppSet::TickTimers),
    );
}

fn enter_playing(mut commands: Commands) {
    commands.trigger(SpawnLevel);
}
