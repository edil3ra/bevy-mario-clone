use crate::Game;

use bevy::{prelude::*, app::PluginGroupBuilder};
use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};
use crate::physics::{Velocity, ForceKind, Force};


struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new()
                .with_system(toggle_fullscreen)
                .with_system(move_camera)
        )
            .add_system(bevy::window::close_on_esc)
            .register_inspectable::<Velocity>()
            .register_inspectable::<ForceKind>()
            .register_inspectable::<Force>();
    }
}

pub struct DebugPlugins;
impl PluginGroup for DebugPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InputPlugin)
            .add(WorldInspectorPlugin::new()

            )

    }
}

fn toggle_fullscreen(
    mut game_resource: ResMut<Game>,
    input: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.primary_mut();
    if input.just_pressed(KeyCode::F12) {
        if game_resource.is_fullscreen {
            window.set_mode(WindowMode::Windowed);
        } else {
            window.set_mode(WindowMode::BorderlessFullscreen);
        }
        game_resource.is_fullscreen = !game_resource.is_fullscreen
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
