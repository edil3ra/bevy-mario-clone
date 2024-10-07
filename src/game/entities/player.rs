use std::sync::Arc;

use crate::{
    config::{FAST_DRAG, SLOW_DRAG},
    game::{
        animations::player_animation::PlayerAnimation,
        assets::{HandleMap, TextureKey},
        movement::{ControllerDirection, MovementController},
        physics::{BoxCollider, Drag, DynamicBoxBundle, Pos},
        traits::{go::Go, solid::Obstruct},
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
    drag_factor_x: f32,
}

#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Running {
    drag_factor_x: f32,
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
    let mut player_command = commands.spawn_empty();
    let player_entity = Arc::new(player_command.id());

    let is_idling = move |In(entity): In<Entity>, query: Query<&Go>| {
        let go = query.get(entity).unwrap();
        if go.distance != 0.0 {
            None
        } else {
            Some(())
        }
    };

    let is_walking = move |In(entity): In<Entity>, query: Query<&MovementController>| {
        let movement = query.get(entity).unwrap();
        if !movement.jumping && !movement.running {
            Some(())
        } else {
            None
        }
    };

    let is_running = move |In(entity): In<Entity>, query: Query<&MovementController>| {
        let movement = query.get(entity).unwrap();
        if !movement.jumping && movement.running {
            Some(())
        } else {
            None
        }
    };

    let is_jumping = move |In(entity): In<Entity>, query: Query<&MovementController>| {
        let movement = query.get(entity).unwrap();
        if movement.jumping {
            return Some(());
        }
        None
    };

    let on_run = {
        let entity = player_entity.clone();
        move |world: &mut World| {
            let running = world.get::<Running>(*entity).unwrap();
            world.get_mut::<Drag>(*entity).unwrap().0.x = running.drag_factor_x;
        }
    };

    let on_walk = {
        let entity = player_entity.clone();
        move |world: &mut World| {
            let walking = world.get::<Walking>(*entity).unwrap();
            world.get_mut::<Drag>(*entity).unwrap().0.x = walking.drag_factor_x;
        }
    };

    let player_state = StateMachine::default()
        .trans_builder(is_walking, |_: &AnyState, _| {
            Some(Walking {
                drag_factor_x: SLOW_DRAG,
            })
        })
        .trans_builder(is_running, |_: &AnyState, _| {
            Some(Running {
                drag_factor_x: FAST_DRAG,
            })
        })
        .trans_builder(is_jumping, |_: &AnyState, _| Some(Jumping { impulse: 10. }))
        .trans_builder(is_idling, |_: &AnyState, _| Some(Idling))
        .trans::<Jumping, _>(done(Some(Done::Success)), Falling)
        .command_on_enter::<Running>(on_run)
        .command_on_enter::<Walking>(on_walk);

    player_command.insert((
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
            distance: 0.,
        },
        Obstruct(false),
        DynamicBoxBundle {
            pos: Pos(Vec2::new(100., 100.)),
            drag: Drag(Vec2::new(FAST_DRAG, 0.)),
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

// impl Command for Running {
//     fn apply(self, world: &mut World) {
//         world.entity(entity)
//         self.speed
//     }
// }

// fn foo(world: &mut World) {
//     world.get(entity)
// }
