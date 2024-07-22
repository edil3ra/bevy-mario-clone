//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::Screen;
use crate::game::{
    assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack, spawn::level::SpawnLevel,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);

}

fn enter_playing(mut commands: Commands) {
    commands.trigger(SpawnLevel);
}
