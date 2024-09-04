use std::time::Duration;

use bevy::prelude::*;

use super::Animate;

#[derive(Resource)]
struct PlayerAtlasLayout(Handle<TextureAtlasLayout>);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Reflect)]
enum Frame {
    #[default]
    Walk1,
    Walk2,
    Flat,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct GoombaAnimation<'a> {
    timer: Timer,
    state: GoombaAnimationState,
    frame: usize,
    frames: &'a [Frame],
}

#[derive(Reflect, PartialEq)]
pub enum GoombaAnimationState {
    Walk,
    Flat,
}

impl<'a> GoombaAnimation<'a> {
    const DURATION_FRAME: Duration = Duration::from_millis(1000 / 4);
    const STATIC_DURATION_FRAME: Duration = Duration::from_millis(10000);

    const WALK_FRAMES: &'a [Frame] = &[Frame::Walk1, Frame::Walk2];
    const FLAT_FRAMES: &'a [Frame] = &[Frame::Flat];

    pub fn walking() -> Self {
        Self {
            timer: Timer::new(Self::DURATION_FRAME, TimerMode::Repeating),
            state: GoombaAnimationState::Walk,
            frames: Self::WALK_FRAMES,
            frame: 0,
        }
    }

    pub fn flating() -> Self {
        Self {
            timer: Timer::new(Self::STATIC_DURATION_FRAME, TimerMode::Repeating),
            state: GoombaAnimationState::Flat,
            frames: Self::WALK_FRAMES,
            frame: 0,
        }
    }

    pub fn new() -> Self {
        Self::walking()
    }

    pub fn update_state(&mut self, state: GoombaAnimationState) {
        if self.state != state {
            match state {
                GoombaAnimationState::Walk => *self = Self::walking(),
                GoombaAnimationState::Flat => *self = Self::flating(),
            }
        }
    }
}

impl<'a> Animate for GoombaAnimation<'a> {
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
