use bevy::prelude::*;

use crate::{
    game::assets::{HandleMap, TextureKey},
    screen::Screen,
};

use super::{EntityKey, Mob, TextureAtlasLayoutEntities};

#[derive(Resource)]
struct PlayerAtlasLayout(Handle<TextureAtlasLayout>);

#[derive(Debug, Clone, Default, PartialEq, Eq, Reflect)]
enum Frame {
    #[default]
    Walk1,
    Walk2,
    Flat,
}

pub const FRAMES_RECT_GOOMBA_BROWN: [[u32; 4]; 3] =
    [[80, 0, 16, 16], [96, 0, 16, 16], [112, 0, 16, 16]];

pub fn spawn_goomba(
    commands: &mut Commands,
    image_handles: &Res<HandleMap<TextureKey>>,
    atlas_layout_handles: &ResMut<TextureAtlasLayoutEntities>,
    key: EntityKey,
    pos_x: u32,
    pos_y: u32,
) {
    commands.spawn((
        Name::new(key.to_string().to_string()),
        Mob {},
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
            index: Frame::Walk1 as usize,
        },
        StateScoped(Screen::Playing),
    ));
}
