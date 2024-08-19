pub mod custom;
pub mod mario_egui;
pub mod tilemap;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((custom::plugin, mario_egui::plugin, tilemap::plugin));
}
