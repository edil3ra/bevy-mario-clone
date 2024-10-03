use crate::{
    game::{
        animations::player_animation::PlayerAnimation,
        assets::{HandleMap, TextureKey},
        movement::{ControllerDirection, MovementController},
        physics::{BoxCollider, Drag, DynamicBoxBundle, Pos},
        traits::go::Go,
    },
    screen::Screen,
};
use bevy::prelude::*;
use seldom_state::prelude::*;

use super::{EntityKey, Player, TextureAtlasLayoutEntities};

#[derive(Debug, Default, Clone, Reflect)]
enum Direction {
    #[default]
    Left,
    Right,
}

#[derive(Debug, Default, Component, Reflect, Clone)]
#[reflect(Component)]
pub struct Idling;

#[derive(Debug, Default, Component, Reflect, Clone)]
#[reflect(Component)]
pub struct Walking {
    speed: f32,
    direction: Direction,
}

#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Running {
    speed: f32,
    direction: Direction,
}

#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Jumping {
    impulse: f32,
}

#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Falling;

pub fn spawn_player(
    commands: &mut Commands,
    image_handles: &Res<HandleMap<TextureKey>>,
    atlas_layout_handles: &ResMut<TextureAtlasLayoutEntities>,
    key: EntityKey,
    pos_x: u32,
    pos_y: u32,
) {
    let is_walking = move |In(entity): In<Entity>, query: Query<&MovementController>| {
        let movement = query.get(entity).unwrap();
        if movement.is_moving() && !movement.jumping && !movement.running {
            match movement.moving {
                ControllerDirection::Idle => return None,
                ControllerDirection::Left => return Some(Direction::Left),
                ControllerDirection::Right => return Some(Direction::Right),
            }
        }
        None
    };

    let is_running = move |In(entity): In<Entity>, query: Query<&MovementController>| {
        let movement = query.get(entity).unwrap();
        if movement.is_moving() && !movement.jumping && movement.running {
            match movement.moving {
                ControllerDirection::Idle => return None,
                ControllerDirection::Left => return Some(Direction::Left),
                ControllerDirection::Right => return Some(Direction::Right),
            }
        }
        None
    };

    let is_jumping = move |In(entity): In<Entity>, query: Query<&MovementController>| {
        let movement = query.get(entity).unwrap();
        if movement.jumping {
            return Some(true);
        }
        None
    };

    let player_state = StateMachine::default()
        .trans_builder(is_walking, |_: &Idling, direction| {
            Some(Walking {
                speed: 1.0,
                direction,
            })
        })
        .trans_builder(is_running, |_: &Idling, direction| {
            Some(Running {
                speed: 2.0,
                direction,
            })
        })
        .trans_builder(is_jumping, |_: &Idling, _jumping| {
            Some(Jumping { impulse: 10. })
        })
        .trans::<Jumping, _>(done(Some(Done::Success)), Falling);

    commands.spawn((
        Name::new(key.to_string().to_string()),
        Player {},
        PlayerAnimation::idling(),
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
        MovementController::default(),
        Idling,
        player_state,
        Go {
            direction: 0,
            heading: 0,
            acceleration: 400.,
            deceleration: 300.,
            drag_factor_x: 1. / 5000.,
            distance: 0.,
        },
        DynamicBoxBundle {
            pos: Pos(Vec2::new(100., 100.)),
            drag: Drag(Vec2::new(1. / 5000., 0.)),
            collider: BoxCollider {
                size: Vec2::new(16., 16.),
            },
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));
}

pub fn jump(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Jumping), Added<Jumping>>,
) {
    for (entity, mut transform, jumping) in &mut query {
        let impulse = jumping.impulse;
        // todo: todo change velocity of mario to make it jump
        commands.entity(entity).insert(Done::Success);
    }
}
