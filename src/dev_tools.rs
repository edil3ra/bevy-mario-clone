use crate::game::SettingsState;
use crate::AppSet;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
    );
    app.add_systems(Update, toggle_fullscreen.in_set(AppSet::RecordInput));
    app.add_systems(Update, move_camera.in_set(AppSet::Update));
}

fn toggle_fullscreen(
    mut settings: ResMut<SettingsState>,
    input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    if input.just_pressed(KeyCode::F12) {
        settings.is_fullscreen = !settings.is_fullscreen;
        window.set_maximized(settings.is_fullscreen);
    }
}

fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut transform = query.get_single_mut().unwrap();
    if keyboard_input.pressed(KeyCode::KeyJ) {
        transform.translation.x += -10.0;
    }

    if keyboard_input.pressed(KeyCode::KeyL) {
        transform.translation.x += 10.0;
    }

    if keyboard_input.pressed(KeyCode::KeyI) {
        transform.translation.y += 10.0;
    }

    if keyboard_input.pressed(KeyCode::KeyK) {
        transform.translation.y += -10.0;
    }

    if keyboard_input.pressed(KeyCode::Digit1) {
        transform.scale *= 0.8;
    }

    if keyboard_input.pressed(KeyCode::Digit2) {
        transform.scale *= 1.2;
    }
}
