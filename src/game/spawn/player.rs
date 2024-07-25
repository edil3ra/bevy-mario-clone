use bevy::prelude::*;

use crate::{
    game::{
        assets::{HandleMap, TextureKey},
        movement::{MovementController, Physics},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    image_handles: Res<HandleMap<TextureKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            texture: image_handles[&TextureKey::Entities].clone_weak(),
            transform: Transform::from_scale(Vec2::splat(8.0).extend(1.0)),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        },
        MovementController::default(),
        Physics {
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));
}
