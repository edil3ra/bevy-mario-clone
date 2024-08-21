pub mod assets;
pub mod movement;
pub mod spawn;
pub mod tiles;

use bevy::prelude::*;

use self::assets::LevelKey;

#[derive(Resource, Default)]
pub struct GameState {
    pub current_level: LevelKey,
}

#[derive(Resource, Default)]
pub struct SettingsState {
    pub is_fullscreen: bool,
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GameState {
        current_level: LevelKey::OneOne,
    });
    app.insert_resource(SettingsState {
        is_fullscreen: false,
    });

    app.add_plugins((assets::plugin, movement::plugin, spawn::plugin));
}
