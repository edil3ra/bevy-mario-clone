use bevy::prelude::*;

use crate::{
    game::{
        animations::goomba_animation::GoombaAnimation,
        assets::{HandleMap, TextureKey},
        physics::{BoxCollider, DynamicBoxBundle, Pos},
        spawn::entities::{EntityKey, Mob, TextureAtlasLayoutEntities},
        traits::solid::Obstruct,
    },
    screen::Screen,
};

pub fn spawn_goomba(
    commands: &mut Commands,
    image_handles: &Res<HandleMap<TextureKey>>,
    atlas_layout_handles: &ResMut<TextureAtlasLayoutEntities>,
    key: EntityKey,
    pos_x: u32,
    pos_y: u32,
) -> Entity {
    commands
        .spawn((
            Name::new(key.to_string().to_string()),
            Mob {},
            GoombaAnimation::walking(),
            SpriteBundle {
                texture: image_handles[&TextureKey::Entities].clone_weak(),
                ..Default::default()
            },
            TextureAtlas {
                layout: atlas_layout_handles.0[&key].clone(),
                index: 0,
            },
            DynamicBoxBundle {
                pos: Pos(Vec2::new(pos_x as f32, pos_y as f32)),
                collider: BoxCollider {
                    size: Vec2::new(16., 16.),
                },
                ..Default::default()
            },
            Obstruct(false),
            StateScoped(Screen::Playing),
        ))
        .id()
}
