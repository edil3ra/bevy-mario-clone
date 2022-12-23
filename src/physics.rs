use bevy::{prelude::*, render::camera::WindowOrigin, time::FixedTimestep};
use crate::config;
pub struct PhysicsPlugin;


#[derive(Component, Default, Clone, Copy)]
pub struct ForceGravity(pub Vec2);
#[derive(Component, Default, Clone, Copy)]
pub struct ForceRun(pub Vec2);
#[derive(Component, Default, Clone, Copy)]
pub struct ForceJump(pub Vec2);
#[derive(Component, Default, Clone, Copy)]
pub struct ForceFriction(pub Vec2);

enum ForceType {
    ForceGravity(Vec2),
    ForceRun(Vec2),
    ForceJump(Vec2),
    ForceFriction(Vec2),
}

struct Forces(Vec<ForceType>);

#[derive(Component, Default, Clone, Copy)]
pub struct Velocity(pub Vec2);



impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(config::TIME_STEP))
                .label("forces")
                .with_system(aplly_forces).label("forces")
        )
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(config::TIME_STEP))
                .after("forces")
                .with_system(aplly_velocity)
        );
    }
}


fn aplly_velocity(mut query: Query<(&mut Transform, &Velocity), With<Velocity>>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += Vec3::from((velocity.0, 0.0));
    }
}


#[allow(clippy::type_complexity)]
fn aplly_forces(
    mut query: Query<
        (&mut Velocity, Option<&ForceGravity>, Option<&ForceRun>),
        Or<(With<ForceGravity>, With<ForceRun>)>,
    >,
) {
    let mut acceleration = Vec2::splat(0.0);
    for (mut velocity, gravity, force_run) in query.iter_mut() {
        if let Some(force) = gravity {
            acceleration += force.0;
        }
        if let Some(force) = force_run {
            acceleration += force.0;
        }
        velocity.0 += acceleration
    }
}


