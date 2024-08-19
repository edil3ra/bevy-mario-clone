use bevy::prelude::*;

use crate::game::SettingsState;
use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, toggle_fullscreen.in_set(AppSet::RecordInput))
        .add_systems(Update, move_camera.in_set(AppSet::Update));
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
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyJ) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyL) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyI) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyK) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyZ) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::KeyX) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 500.;
        transform.translation.z = z;
    }
}
