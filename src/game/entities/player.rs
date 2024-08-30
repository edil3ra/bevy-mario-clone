use std::{borrow::Borrow, time::Duration};

use bevy::prelude::*;
use seldom_state::prelude::*;

use crate::{
    game::{
        animate::Animate,
        assets::{HandleMap, TextureKey},
        movement::{ControllerDirection, MovementController, Physics},
    },
    screen::Screen,
};

use super::{EntityKey, Player, TextureAtlasLayoutEntities};

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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Reflect)]
enum Frame {
    #[default]
    Idle = 0,
    Run1 = 1,
    Run2 = 2,
    Run3 = 3,
    Break = 4,
    Jump = 5,
    Die = 6,
    Climb1 = 7,
    Climb2 = 8,
    Swim1 = 9,
    Swim2 = 10,
    Swim3 = 11,
    Swim4 = 12,
    Swim5 = 13,
    IdleLarge = 14,
    Run1Large = 15,
    Run2Large = 16,
    Run3Large = 17,
    BreakLarge = 18,
    JumpLarge = 19,
    CrouchLarge = 20,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerAnimation<'a> {
    timer: Timer,
    state: PlayerAnimationState,
    frame: usize,
    frames: &'a [Frame],
}

#[derive(Reflect, PartialEq)]
pub enum PlayerAnimationState {
    Idle,
    Run,
    Break,
    Jump,
    Die,
    Climb,
    Swim,
    IdleLarge,
    RunneLarge,
    BreakeLarge,
    JumpeLarge,
    ClimbeLarge,
    SwimmeLarge,
    CroucheLarge,
}

impl<'a> PlayerAnimation<'a> {
    const SHORT_DURATION: Duration = Duration::from_millis(1000 / 4);
    const LONG_DURATION: Duration = Duration::from_millis(1000 / 6);
    const VERY_LONG_DURATION: Duration = Duration::from_millis(10000);

    const IDLE_FRAMES: &'a [Frame] = &[Frame::Idle];
    const RUN_FRAMES: &'a [Frame] = &[Frame::Run1, Frame::Run2, Frame::Run3];
    const BREAK_FRAMES: &'a [Frame] = &[Frame::Break];
    const JUMP_FRAMES: &'a [Frame] = &[Frame::Jump];
    const DIE_FRAMES: &'a [Frame] = &[Frame::Die];
    const CLIMB_FRAMES: &'a [Frame] = &[Frame::Climb1, Frame::Climb2];
    const SWIM_FRAMES: &'a [Frame] = &[Frame::Swim1, Frame::Swim2, Frame::Swim3, Frame::Swim4];
    const IDLE_LARGE_FRAMES: &'a [Frame] = &[Frame::IdleLarge];
    const RUN_LARGE_FRAMES: &'a [Frame] = &[Frame::Run1Large, Frame::Run2Large, Frame::Run3Large];
    const BREAK_LARGE_FRAMES: &'a [Frame] = &[Frame::BreakLarge];
    const JUMP_LARGE_FRAMES: &'a [Frame] = &[Frame::JumpLarge];
    const CLIMB_LARGE_FRAMES: &'a [Frame] = &[Frame::Climb1, Frame::Climb2];
    const SWIM_LARGE_FRAMES: &'a [Frame] =
        &[Frame::Swim1, Frame::Swim2, Frame::Swim3, Frame::Swim4];
    const CROUCH_LARGE_FRAMES: &'a [Frame] = &[Frame::CrouchLarge];

    fn idling() -> Self {
        Self {
            timer: Timer::new(Self::VERY_LONG_DURATION, TimerMode::Repeating),
            state: PlayerAnimationState::Idle,
            frames: Self::IDLE_FRAMES,
            frame: 0,
        }
    }

    fn running() -> Self {
        Self {
            timer: Timer::new(Self::LONG_DURATION, TimerMode::Repeating),
            state: PlayerAnimationState::Run,
            frames: Self::RUN_FRAMES,
            frame: 0,
        }
    }

    pub fn new() -> Self {
        Self::idling()
    }

    pub fn update_state(&mut self, state: PlayerAnimationState) {
        if self.state != state {
            match state {
                PlayerAnimationState::Idle => *self = Self::idling(),
                PlayerAnimationState::Run => *self = Self::running(),
                PlayerAnimationState::Break => todo!(),
                PlayerAnimationState::Jump => todo!(),
                PlayerAnimationState::Die => todo!(),
                PlayerAnimationState::Climb => todo!(),
                PlayerAnimationState::Swim => todo!(),
                PlayerAnimationState::IdleLarge => todo!(),
                PlayerAnimationState::RunneLarge => todo!(),
                PlayerAnimationState::BreakeLarge => todo!(),
                PlayerAnimationState::JumpeLarge => todo!(),
                PlayerAnimationState::ClimbeLarge => todo!(),
                PlayerAnimationState::SwimmeLarge => todo!(),
                PlayerAnimationState::CroucheLarge => todo!(),
            }
        }
    }
}

impl<'a> Animate for PlayerAnimation<'a> {
    fn changed(&self) -> bool {
        self.timer.finished()
    }

    fn get_atlas_index(&self) -> usize {
        self.frames[self.frame] as usize
    }

    fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.finished() {
            return;
        }
        self.frame = (self.frame + 1) % self.frames.len()
    }
}

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
        .trans::<Jumping, _>(done(Some(Done::Success)), Falling)

        
        ;

    commands.spawn((
        Name::new(key.to_string().to_string()),
        Player {},
        PlayerAnimation::running(),
        Idling,
        player_state,
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


pub fn jump(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Jumping), Added<Jumping> >,
) {
    for (entity, mut transform, jumping) in &mut query {
        let impulse = jumping.impulse;
        // todo: todo change velocity of mario to make it jump
        dbg!("enter");
        commands.entity(entity).insert(Done::Success);
    }
}

