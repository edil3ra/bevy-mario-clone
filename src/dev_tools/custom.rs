use bevy::prelude::*;

use crate::game::physics::DT;
use crate::game::SettingsState;
use crate::AppSet;

#[derive(Debug, Default, PartialEq, Resource)]
pub struct DebugMode(pub bool);

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DebugMode>();

    app.add_systems(
        Update,
        (toggle_fullscreen, toogle_debug).in_set(AppSet::RecordInput),
    );

    app.add_systems(
        Update,
        move_camera
            .run_if(resource_exists::<DebugMode>.and_then(resource_equals(DebugMode(true)))),
    );
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

fn toogle_debug(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut time: ResMut<Time<Virtual>>,
    mut debug_mode: ResMut<DebugMode>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        debug_mode.0 = !debug_mode.0;
        if time.is_paused() {
            time.unpause();
        } else {
            time.pause();
        }
    }
}

fn move_camera(
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
        transform.translation += direction * 500. * DT ;
        transform.translation.z = z;
    }
}
