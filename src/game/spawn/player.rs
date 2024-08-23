use bevy::prelude::*;

use crate::{
    game::{
        assets::{HandleMap, TextureKey},
        movement::{MovementController, Physics},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), setup_player);
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Resource)]
struct PlayerAtlasLayout(Handle<TextureAtlasLayout>);

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player {
    current_frame: Frame,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Reflect)]
enum Frame {
    #[default]
    Idle,
    Run1,
    Run2,
    Run3,
    Break,
    Jump,
    Die,
    Climb1,
    Climb2,
    Swim1,
    Swim2,
    Swim3,
    Swim4,
    Swim5,
    Idlelarge,
    Run1large,
    Run2large,
    Run3large,
    BreakLarge,
    JumpLarge,
    CrouchLarge,
}

const FRAMES_RECT: [[u32; 4]; 21] = [
    [0, 88, 16, 16],
    [16, 88, 16, 16],
    [32, 88, 16, 16],
    [48, 88, 16, 16],
    [64, 88, 16, 16],
    [80, 88, 16, 16],
    [96, 88, 16, 16],
    [0, 104, 16, 16],
    [16, 104, 16, 16],
    [32, 104, 16, 16],
    [48, 104, 16, 16],
    [64, 104, 16, 16],
    [80, 104, 16, 16],
    [96, 104, 16, 16],
    [112, 88, 16, 32],
    [128, 88, 16, 32],
    [144, 88, 16, 32],
    [160, 88, 16, 32],
    [176, 88, 16, 32],
    [192, 88, 16, 32],
    [0, 120, 16, 32],
];

fn setup_player(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut layout = TextureAtlasLayout::new_empty(UVec2::new(29 * 8, 29 * 8));
    for frame_rect in FRAMES_RECT {
        layout.add_texture(URect::new(
            frame_rect[0],
            frame_rect[1],
            frame_rect[0] + frame_rect[2],
            frame_rect[1] + frame_rect[3],
        ));
    }
    let handle = texture_atlas_layouts.add(layout);
    commands.insert_resource(PlayerAtlasLayout(handle));
}

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    image_handles: Res<HandleMap<TextureKey>>,
    atlas_handle: ResMut<PlayerAtlasLayout>,
) {
    commands.spawn((
        Name::new("Player"),
        Player {
            current_frame: Frame::Idle,
        },
        SpriteBundle {
            texture: image_handles[&TextureKey::Entities].clone_weak(),
            transform: Transform {
                translation: Vec3::new(100., 100., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        TextureAtlas {
            layout: atlas_handle.0.clone(),
            index: 2,
        },
        MovementController::default(),
        Physics {
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));
}
