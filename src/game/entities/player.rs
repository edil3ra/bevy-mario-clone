use bevy::prelude::*;

use crate::{
    game::{
        assets::{HandleMap, TextureKey},
        movement::{MovementController, Physics},
    },
    screen::Screen,
};

use super::{EntityKey, Player, TextureAtlasLayoutEntities};

#[derive(Resource)]
struct PlayerAtlasLayout(Handle<TextureAtlasLayout>);

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

pub const FRAMES_RECT_PLAYER: [[u32; 4]; 21] = [
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

pub fn spawn_player(
    commands: &mut Commands,
    image_handles: &Res<HandleMap<TextureKey>>,
    atlas_layout_handles: &ResMut<TextureAtlasLayoutEntities>,
    key: EntityKey,
    pos_x: u32,
    pos_y: u32,
) {
    commands.spawn((
        Name::new(key.to_string().to_string()),
        Player {},
        SpriteBundle {
            texture: image_handles[&TextureKey::Entities].clone_weak(),
            transform: Transform {
                translation: Vec3::new(pos_x as f32, pos_y as f32, 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        TextureAtlas {
            layout: atlas_layout_handles.0[&key].clone(),
            index: Frame::Idle as usize,
        },
        MovementController::default(),
        Physics {
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));
}
