// mod utils;
// use utils::*;

use std::time::Duration;

use bevy::prelude::*;

pub const DELTA_TIME: f32 = 1. / 60.;
const COLLISION_PAIR_VEL_MARGIN_FACTOR: f32 = 2. * DELTA_TIME;

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
pub struct Contacts(pub Vec<(Entity, Entity, Vec2)>);

#[derive(Resource, Debug, Default)]
pub(crate) struct CollisionPairs(pub Vec<(Entity, Entity)>);

#[derive(Resource)]
pub struct Gravity(pub Vec2);
impl Default for Gravity {
    fn default() -> Self {
        Self(Vec2::new(0., -9.81))
    }
}

#[derive(Component, Debug, Default)]
pub struct Aabb {
    pub min: Vec2,
    pub max: Vec2,
}

impl Aabb {
    pub fn intersects(&self, other: &Self) -> bool {
        self.max.x >= other.min.x
            && self.max.y >= other.min.y
            && self.min.x <= other.max.x
            && self.min.y <= other.max.y
    }
}

pub struct Contact {
    pub penetration: f32,
    pub normal: Vec2,
}

#[derive(Component, Debug)]
pub struct BoxCollider {
    pub size: Vec2,
}

impl Default for BoxCollider {
    fn default() -> Self {
        Self { size: Vec2::ONE }
    }
}

#[derive(Bundle, Default)]
pub struct DynamicBoxBundle {
    pub pos: Pos,
    pub prev_pos: PrevPos,
    pub mass: Mass,
    pub collider: BoxCollider,
    pub vel: Vel,
    pub pre_solve_vel: PreSolveVel,
    pub restitution: Restitution,
    pub aabb: Aabb,
}

impl DynamicBoxBundle {
    pub fn new_with_pos_and_vel(pos: Vec2, vel: Vec2) -> Self {
        Self {
            pos: Pos(pos),
            prev_pos: PrevPos(pos - vel * SUB_DT),
            vel: Vel(vel),
            ..Default::default()
        }
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum Step {
    UpdateAABB,
    CollectCollisionPairs,
    Integrate,
    SolvePositions,
    UpdateVelocities,
    SolveVelocities,
    SyncTransform,
}

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(
        FixedUpdate,
        (
            Step::UpdateAABB,
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
    app.init_resource::<CollisionPairs>();
    app.init_resource::<Gravity>();
    app.insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f32(
        DELTA_TIME,
    )));

    app.add_systems(FixedUpdate, (update_aabb_box).in_set(Step::UpdateAABB));
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

fn update_aabb_box(mut query: Query<(&mut Aabb, &Pos, &Vel, &BoxCollider)>) {
    for (mut aabb, pos, vel, rbox) in query.iter_mut() {
        let margin = COLLISION_PAIR_VEL_MARGIN_FACTOR * vel.0.length();
        let half_extents = rbox.size / 2. + Vec2::splat(margin);
        aabb.min = pos.0 - half_extents;
        aabb.max = pos.0 + half_extents;
    }
}

fn collect_collision_pairs(
    query: Query<(Entity, &Aabb)>,
    mut collision_pairs: ResMut<CollisionPairs>,
) {
    collision_pairs.0.clear();

    unsafe {
        for (entity_a, aabb_a) in query.iter_unsafe() {
            for (entity_b, aabb_b) in query.iter_unsafe() {
                if entity_a <= entity_b {
                    continue;
                }
                if aabb_a.intersects(aabb_b) {
                    collision_pairs.0.push((entity_a, entity_b));
                }
            }
        }
    }
}

pub fn box_box(pos_a: Vec2, size_a: Vec2, pos_b: Vec2, size_b: Vec2) -> Option<Contact> {
    let half_a = size_a / 2.;
    let half_b = size_b / 2.;
    let ab = pos_b - pos_a;
    let overlap = (half_a + half_b) - ab.abs(); // exploit symmetry
    if overlap.x < 0. || overlap.y < 0. {
        None
    } else if overlap.x < overlap.y {
        // closer to vertical edge
        Some(Contact {
            penetration: overlap.x,
            normal: Vec2::X * ab.x.signum(),
        })
    } else {
        // closer to horizontal edge
        Some(Contact {
            penetration: overlap.y,
            normal: Vec2::Y * ab.y.signum(),
        })
    }
}

fn integrate(
    mut query: Query<(&mut Pos, &mut PrevPos, &mut Vel, &mut PreSolveVel, &Mass)>, // <-- new
    gravity: Res<Gravity>,
) {
    for (mut pos, mut prev_pos, mut vel, mut pre_solve_vel, mass) in query.iter_mut() {
        prev_pos.0 = pos.0;

        let gravitation_force = mass.0 * gravity.0;
        let external_forces = gravitation_force;
        vel.0 += DELTA_TIME * external_forces / mass.0;
        pos.0 += DELTA_TIME * vel.0;
        pre_solve_vel.0 = vel.0;
    }
}

fn constrain_body_positions(
    pos_a: &mut Pos,
    pos_b: &mut Pos,
    mass_a: &Mass,
    mass_b: &Mass,
    n: Vec2,
    penetration_depth: f32,
) {
    let w_a = 1. / mass_a.0;
    let w_b = 1. / mass_b.0;
    let w_sum = w_a + w_b;
    let pos_impulse = n * (-penetration_depth / w_sum);
    pos_a.0 += pos_impulse * w_a;
    pos_b.0 -= pos_impulse * w_b;
}

fn solve_pos(
    mut query: Query<(&mut Pos, &BoxCollider, &Mass)>,
    mut contacts: ResMut<Contacts>,
    collision_pairs: Res<CollisionPairs>,
) {
    for (entity_a, entity_b) in collision_pairs.0.iter().cloned() {
        let ((mut pos_a, box_a, mass_a), (mut pos_b, box_b, mass_b)) = unsafe {
            (
                query.get_unchecked(entity_a).unwrap(),
                query.get_unchecked(entity_b).unwrap(),
            )
        };

        if let Some(Contact {
            normal,
            penetration,
        }) = box_box(pos_a.0, box_a.size, pos_b.0, box_b.size)
        {
            constrain_body_positions(&mut pos_a, &mut pos_b, mass_a, mass_b, normal, penetration);
            contacts.0.push((entity_a, entity_b, normal));
        }
    }
}

fn update_vel(mut query: Query<(&Pos, &PrevPos, &mut Vel)>) {
    for (pos, prev_pos, mut vel) in query.iter_mut() {
        vel.0 = (pos.0 - prev_pos.0) / DELTA_TIME;
    }
}

fn solve_vel(
    query: Query<(&mut Vel, &PreSolveVel, &Pos, &Mass, &Restitution)>,
    contacts: Res<Contacts>,
) {
    for (entity_a, entity_b, n) in contacts.0.iter().cloned() {
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
