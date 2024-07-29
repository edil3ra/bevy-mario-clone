mod config;
mod dev_tools;
mod game;
mod screen;
mod ui;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppSet {
    TickTimers,
    RecordInput,
    Update,
}

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (config::WINDOW_WIDTH, config::WINDOW_HEIGHT).into(),
                        // mode: bevy::window::WindowMode::Windowed,
                        mode: bevy::window::WindowMode::BorderlessFullscreen,
                        title: "Mario".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );

        app.add_plugins((game::plugin, screen::plugin, ui::plugin));
        app.add_systems(Startup, spawn_camera);

        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

fn spawn_camera(mut commands: Commands) {
    let scale_factor = config::WINDOW_HEIGHT / config::TILE_SIZE / config::TILE_MAX_HEIGHT;
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(
                config::WINDOW_WIDTH  / 2.0 / scale_factor - config::TILE_SIZE/2.0,
                config::WINDOW_HEIGHT  / 2.0 / scale_factor - config::TILE_SIZE/2.0,
                1.0,
            ),
            scale: Vec3::new(1.0 / scale_factor, 1.0 / scale_factor, 2.0),
            ..Default::default()
        },
        projection: OrthographicProjection {
            ..Default::default()
        },
        ..Default::default()
    });
}

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
