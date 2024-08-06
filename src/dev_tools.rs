use crate::game::SettingsState;
use crate::AppSet;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::{color::palettes, math::Vec4Swizzles};
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
    );
    app.add_systems(Update, toggle_fullscreen.in_set(AppSet::RecordInput));
    app.add_systems(Update, move_camera.in_set(AppSet::Update));

    app.init_resource::<CursorPos>()
        .init_resource::<FontHandle>();

    app.observe(spawn_tile_labels);

    app.add_systems(First, (update_cursor_pos).chain())
        .add_systems(Update, highlight_tile_labels);
}

impl FromWorld for FontHandle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("fonts/FiraSans-Bold.ttf"))
    }
}

#[derive(Event, Debug)]
pub struct SpawnTileLabelsDebug;

#[derive(Deref, Resource)]
pub struct FontHandle(Handle<Font>);

#[derive(Component)]
struct HighlightedLabel;

#[derive(Component)]
struct TileLabel(Entity);

#[derive(SystemSet, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct SpawnTilemapSet;

#[derive(Resource)]
pub struct CursorPos(Vec2);
impl Default for CursorPos {
    fn default() -> Self {
        Self(Vec2::new(-1000.0, -1000.0))
    }
}

// We need to keep the cursor position updated based on any `CursorMoved` events.
pub fn update_cursor_pos(
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    for cursor_moved in cursor_moved_events.read() {
        // To get the mouse's world position, we have to transform its window position by
        // any transforms on the camera. This is done by projecting the cursor position into
        // camera space (world space).
        for (cam_t, cam) in camera_q.iter() {
            if let Some(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
                *cursor_pos = CursorPos(pos);
            }
        }
    }
}

// Generates tile position labels of the form: `(tile_pos.x, tile_pos.y)`
fn spawn_tile_labels(
    _trigger: Trigger<SpawnTileLabelsDebug>,
    mut commands: Commands,
    tilemap_q: Query<(&Transform, &TilemapType, &TilemapGridSize, &TileStorage)>,
    tile_q: Query<&mut TilePos>,
    font_handle: Res<FontHandle>,
) {
    let text_style = TextStyle {
        font: font_handle.clone(),
        font_size: 20.0,
        color: Color::BLACK,
    };
    let text_justify = JustifyText::Center;
    for (map_transform, map_type, grid_size, tilemap_storage) in tilemap_q.iter() {
        for tile_entity in tilemap_storage.iter().flatten() {
            let tile_pos = tile_q.get(*tile_entity).unwrap();
            let tile_center = tile_pos.center_in_world(grid_size, map_type).extend(1.0);
            let transform = *map_transform * Transform::from_translation(tile_center);

            let label_entity = commands
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        format!("{}, {}", tile_pos.x, tile_pos.y),
                        text_style.clone(),
                    )
                    .with_justify(text_justify),
                    transform,
                    ..default()
                })
                .id();
            commands
                .entity(*tile_entity)
                .insert(TileLabel(label_entity));
        }
    }
}

// This is where we check which tile the cursor is hovered over.
fn highlight_tile_labels(
    mut commands: Commands,
    cursor_pos: Res<CursorPos>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
    highlighted_tiles_q: Query<Entity, With<HighlightedLabel>>,
    tile_label_q: Query<&TileLabel>,
    mut text_q: Query<&mut Text>,
) {
    // Un-highlight any previously highlighted tile labels.
    for highlighted_tile_entity in highlighted_tiles_q.iter() {
        if let Ok(label) = tile_label_q.get(highlighted_tile_entity) {
            if let Ok(mut tile_text) = text_q.get_mut(label.0) {
                for section in tile_text.sections.iter_mut() {
                    section.style.color = Color::BLACK;
                }
                commands
                    .entity(highlighted_tile_entity)
                    .remove::<HighlightedLabel>();
            }
        }
    }

    for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter() {
        // Grab the cursor position from the `Res<CursorPos>`
        let cursor_pos: Vec2 = cursor_pos.0;
        // We need to make sure that the cursor's world position is correct relative to the map
        // due to any map transformation.
        let cursor_in_map_pos: Vec2 = {
            // Extend the cursor_pos vec3 by 0.0 and 1.0
            let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };
        // Once we have a world position we can transform it into a possible tile position.
        if let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
        {
            // Highlight the relevant tile's label
            if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                if let Ok(label) = tile_label_q.get(tile_entity) {
                    if let Ok(mut tile_text) = text_q.get_mut(label.0) {
                        for section in tile_text.sections.iter_mut() {
                            section.style.color = palettes::tailwind::RED_600.into();
                        }
                        commands.entity(tile_entity).insert(HighlightedLabel);
                    }
                }
            }
        }
    }
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
    // let mut transform = query.get_single_mut().unwrap();
    // if keyboard_input.pressed(KeyCode::KeyJ) {
    //     transform.translation.x += -10.0;
    // }

    // if keyboard_input.pressed(KeyCode::KeyL) {
    //     transform.translation.x += 10.0;
    // }

    // if keyboard_input.pressed(KeyCode::KeyI) {
    //     transform.translation.y += 10.0;
    // }

    // if keyboard_input.pressed(KeyCode::KeyK) {
    //     transform.translation.y += -10.0;
    // }

    // if keyboard_input.pressed(KeyCode::Digit1) {
    //     transform.scale *= 0.8;
    // }

    // if keyboard_input.pressed(KeyCode::Digit2) {
    //     transform.scale *= 1.2;
    // }

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
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}
