use std::time::Duration;

use bevy::prelude::*;

pub const DELTA_TIME: f32 = 1. / 60.;

#[derive(Component, Debug, Default)]
pub struct Pos(pub Vec2);

#[derive(Component, Debug, Default)]
pub struct PrevPos(pub Vec2);

#[derive(Component, Debug)]
pub struct Mass(pub f32);
impl Default for Mass {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component, Debug, Default)]
pub struct PreSolveVel(pub Vec2);

#[derive(Component, Debug, Default)]
pub struct Vel(pub Vec2);

#[derive(Component, Debug)]
pub struct Restitution(pub f32);

impl Default for Restitution {
    fn default() -> Self {
        Self(0.3)
    }
}

#[derive(Resource, Debug, Default)]
pub struct Contacts(pub Vec<(Entity, Entity)>);

#[derive(Resource)]
pub struct Gravity(pub Vec2);
impl Default for Gravity {
    fn default() -> Self {
        Self(Vec2::new(0., -9.81))
    }
}

#[derive(Component, Debug)]
pub struct CircleCollider {
    pub radius: f32,
}

impl Default for CircleCollider {
    fn default() -> Self {
        Self { radius: 0.5 }
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum Step {
    CollectCollisionPairs,
    Integrate,
    SolvePositions,
    UpdateVelocities,
    SolveVelocities,
    SyncTransform,
}

pub struct ParticleBundle {
    pub pos: Pos,
    pub prev_pos: PrevPos,
    pub mass: Mass, // <-- new
}

// impl ParticleBundle {
//     pub fn new_with_pos_and_vel(pos: Vec2, vel: Vec2) -> Self {
//         Self {
//             pos: Pos(pos),
//             prev_pos: PrevPos(pos - vel * DELTA_TIME),
//             ..Default::default(), // <-- new
//         }
//     }
// }

// impl Default for Mass {
//     fn default() -> Self {
//         Self(1.) // Default to 1 kg
//     }
// }

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(
        FixedUpdate,
        (
            Step::CollectCollisionPairs,
            Step::Integrate,
            Step::SolvePositions,
            Step::UpdateVelocities,
            Step::SolveVelocities,
            Step::SyncTransform,
        )
            .chain(),
    );
    app.init_resource::<Contacts>();
    app.init_resource::<Gravity>();
    app.insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f32(
        DELTA_TIME,
    )));

    app.add_systems(
        FixedUpdate,
        (collect_collision_pairs).in_set(Step::CollectCollisionPairs),
    );
    app.add_systems(FixedUpdate, (integrate).in_set(Step::Integrate));
    app.add_systems(FixedUpdate, (solve_pos).in_set(Step::SolvePositions));
    app.add_systems(FixedUpdate, (update_vel).in_set(Step::UpdateVelocities));
    app.add_systems(FixedUpdate, (solve_vel).in_set(Step::SolveVelocities));
    app.add_systems(FixedUpdate, (sync_transforms).in_set(Step::SyncTransform));
}

// Moves objects in the physics world
pub fn simulate(mut query: Query<(&mut Pos, &mut Vel, &mut PrevPos, &Mass)>) {
    for (mut pos, mut prev_pos, mut vel, mass) in query.iter_mut() {
        let gravity = Vec2::new(0., -9.81);
        let gravitation_force = mass.0 * gravity;
        let external_forces = gravitation_force;
        vel.0 += DELTA_TIME * external_forces / mass.0;
        pos.0 += DELTA_TIME * vel.0;
    }

    for (pos, prev_pos, mut vel, _mass) in query.iter_mut() {
        vel.0 = (pos.0 - prev_pos.0) / DELTA_TIME;
    }
}

fn collect_collision_pairs() {}

fn integrate(
    mut query: Query<(&mut Pos, &mut PrevPos, &mut Vel, &mut PreSolveVel, &Mass)>, // <-- new
    gravity: Res<Gravity>,
) {
    for (mut pos, mut prev_pos, mut vel, mut pre_solve_vel, mass) in query.iter_mut() {
        // <-- new
        prev_pos.0 = pos.0;

        let gravitation_force = mass.0 * gravity.0;
        let external_forces = gravitation_force;
        vel.0 += DELTA_TIME * external_forces / mass.0;
        pos.0 += DELTA_TIME * vel.0;
        pre_solve_vel.0 = vel.0; // <-- new
    }
}

fn solve_pos(mut query: Query<(Entity, &mut Pos, &CircleCollider, &Mass)>, mut contacts: ResMut<Contacts>) {
    contacts.0.clear();
    let mut combinations = query.iter_combinations_mut();

    while let Some([(entity_a, mut pos_a, circle_a, mass_a), (entity_b, mut pos_b, circle_b, mass_b)]) =
        combinations.fetch_next()
    {
        let ab = pos_b.0 - pos_a.0;
        let combined_radius = circle_a.radius + circle_b.radius;
        if ab.length_squared() < combined_radius * combined_radius {
            let penetration_depth = combined_radius - ab.length();
            let n = ab.normalize();
            let w_a = 1. / mass_a.0;
            let w_b = 1. / mass_b.0;
            let w_sum = w_a + w_b;

            pos_a.0 -= n * penetration_depth * w_a / w_sum;
            pos_b.0 += n * penetration_depth * w_b / w_sum;
            contacts.0.push((entity_a, entity_b));
        }
    }
}

fn update_vel(mut query: Query<(&Pos, &PrevPos, &mut Vel)>) {
    for (pos, prev_pos, mut vel) in query.iter_mut() {
        vel.0 = (pos.0 - prev_pos.0) / DELTA_TIME;
    }
}


fn solve_vel(query: Query<(&mut Vel, &PreSolveVel, &Pos, &Mass, &Restitution)>, contacts: Res<Contacts>) {
    for (entity_a, entity_b) in contacts.0.iter().cloned() {
        let (
            (mut vel_a, pre_solve_vel_a, pos_a, mass_a, restitution_a),
            (mut vel_b, pre_solve_vel_b, pos_b, mass_b, restitution_b),
        ) = unsafe {
            // Ensure safety
            assert!(entity_a != entity_b);
            (
                query.get_unchecked(entity_a).unwrap(),
                query.get_unchecked(entity_b).unwrap(),
            )
        };
        let n = (pos_b.0 - pos_a.0).normalize();
        let pre_solve_relative_vel = pre_solve_vel_a.0 - pre_solve_vel_b.0;
        let pre_solve_normal_vel = Vec2::dot(pre_solve_relative_vel, n);

        let relative_vel = vel_a.0 - vel_b.0;
        let normal_vel = Vec2::dot(relative_vel, n);
        let restitution = (restitution_a.0 + restitution_b.0) / 2.;

        let w_a = 1. / mass_a.0;
        let w_b = 1. / mass_b.0;
        let w_sum = w_a + w_b;

        vel_a.0 += n * (-normal_vel - restitution * pre_solve_normal_vel) * w_a / w_sum;
        vel_b.0 -= n * (-normal_vel - restitution * pre_solve_normal_vel) * w_b / w_sum;
    }
}


pub fn sync_transforms(mut query: Query<(&mut bevy::transform::components::Transform, &Pos)>) {
    for (mut transform, pos) in query.iter_mut() {
        transform.translation = pos.0.extend(0.);
    }
}
