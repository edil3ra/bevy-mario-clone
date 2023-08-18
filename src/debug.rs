use crate::Game;
use bevy::input::common_conditions::input_toggle_active;
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_camera, toggle_fullscreen));
    }
}

pub struct DebugPlugins;
impl PluginGroup for DebugPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InputPlugin)
            .add(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)))
    }
}

fn toggle_fullscreen(
    mut game_resource: ResMut<Game>,
    input: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    if input.just_pressed(KeyCode::F12) {
        game_resource.is_fullscreen = !game_resource.is_fullscreen;
        window.set_maximized(game_resource.is_fullscreen);
    }
}

fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut transform = query.get_single_mut().unwrap();
    if keyboard_input.pressed(KeyCode::G) {
        transform.translation.x += -10.0;
    }

    if keyboard_input.pressed(KeyCode::C) {
        transform.translation.x += 10.0;
    }

    if keyboard_input.pressed(KeyCode::R) {
        transform.translation.y += 10.0;
    }

    if keyboard_input.pressed(KeyCode::L) {
        transform.translation.y += -10.0;
    }

    if keyboard_input.pressed(KeyCode::Plus) {
        transform.scale *= 0.8;
    }

    if keyboard_input.pressed(KeyCode::Minus) {
        transform.scale *= 1.2;
    }
}
