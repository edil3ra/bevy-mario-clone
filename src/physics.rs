use bevy::utils::HashSet;
use std::hash::{Hash, Hasher};

use crate::config;
use bevy::{prelude::*, time::FixedTimestep};
use bevy_inspector_egui::{Inspectable};

#[derive(Component, Inspectable, Default, Clone, Debug, Copy, Hash, Eq, PartialEq)]
pub enum ForceKind {
    #[default]
    Run,
    Gravity,
    Jump,
    Friction,
}

#[derive(Component, Inspectable, Default, Clone, Copy)]
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
pub struct Body {
    pub mass: f64,
    pub velocity: Vec2,
    pub forces: HashSet<Force>,
}

impl Body {
    pub fn new(mass: f64, inital_velocity: Vec2, forces: HashSet<Force>) -> Self {
        Body {
            mass,
            velocity: inital_velocity,
            forces,
        }
    }
}

#[derive(Debug, Resource, Default)]
pub struct Physics {
    gravity: Vec2
}

pub struct PhysicsPlugin {
    pub init_gravity: Vec2
}

impl Default for PhysicsPlugin {
    fn default() -> Self {
        PhysicsPlugin {
            init_gravity: Vec2::new(0., -1.0),
        }
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HashSet<Force>>();

        app.insert_resource(Physics {
            gravity: self.init_gravity
        });
        
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



fn aplly_velocity(mut query: Query<(&mut Transform, &Body), With<Body>>, dt: Res<Time>) {
    for (mut transform, body) in query.iter_mut() {
        transform.translation += Vec3::from((body.velocity, 0.0)) * dt.elapsed_seconds();
    }
}

fn aplly_forces(physics_res: Res<Physics>, mut query: Query<&mut Body>) {
    for mut body in query.iter_mut() {
        let body_force: Vec2 = body.forces.iter().map(|x| x.vec).sum();
        let total_force: Vec2 = body_force + physics_res.gravity;
        let acceleration = total_force / 1.0;
        body.velocity += acceleration
    }
}
