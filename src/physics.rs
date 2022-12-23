use std::{collections::HashSet, hash::Hash, hash::Hasher};

use bevy::{prelude::*, time::FixedTimestep};

use crate::config;


#[derive(Component, Default, Clone, Debug, Copy, Hash, Eq, PartialEq)]
pub enum ForceKind {
    #[default]
    Run,
    Gravity,
    Jump,
    Friction,
}

#[derive(Component, Default, Clone)]
pub struct Force {
    kind: ForceKind,
    vec: Vec2,
}

impl Force {
    pub fn new(kind: ForceKind, vec: Vec2) -> Self {
        Force { kind, vec }
    }
}

impl Hash for Force {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
    }
}

impl PartialEq for Force {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}
impl Eq for Force {}

#[derive(Component, Default, Clone)]
pub struct Forces(pub HashSet<Force>);

#[derive(Component, Default, Clone, Copy)]
pub struct Velocity(pub Vec2);

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(config::TIME_STEP))
                .label("forces")
                .with_system(aplly_forces)
                .label("forces"),
        )
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(config::TIME_STEP))
                .after("forces")
                .with_system(aplly_velocity),
        );
    }
}

fn aplly_velocity(mut query: Query<(&mut Transform, &Velocity), With<Velocity>>, dt: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += Vec3::from((velocity.0, 0.0));
    }
}

fn aplly_forces(mut query: Query<(&mut Velocity, &mut Forces)>, dt: Res<Time>) {
    for (mut velocity, forces) in query.iter_mut() {
        let total_force: Vec2 = forces.0.iter().map(|x| x.vec).sum();
        let acceleration = total_force / 1.0;
        velocity.0 += acceleration * dt.elapsed_seconds();
    }
}
