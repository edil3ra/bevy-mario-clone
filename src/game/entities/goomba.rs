use bevy::prelude::*;

use crate::{
    game::{animations::goomba_animation::GoombaAnimation, assets::{HandleMap, TextureKey}},
    screen::Screen,
};

use super::{EntityKey, Mob, TextureAtlasLayoutEntities};

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
        GoombaAnimation::walking(),
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
            index: 0,
        },
        StateScoped(Screen::Playing),
    ));
}
