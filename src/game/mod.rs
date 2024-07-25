use bevy::prelude::*;

pub mod assets;
mod movement;
pub mod spawn;

#[derive(Resource, Default)]
pub struct GameState {
    pub current_level: String,
}

#[derive(Resource, Default)]
pub struct SettingsState {
    pub is_fullscreen: bool,
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GameState {
        current_level: "1-1".to_string(),
    });
    app.insert_resource(SettingsState {
        is_fullscreen: false,
    });

    app.add_plugins((assets::plugin, movement::plugin, spawn::plugin));
}
