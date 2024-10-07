use super::components::*;
use super::resources::*;
use super::COLLISION_PAIR_VEL_MARGIN_FACTOR;
use super::DT;
use bevy::prelude::*;

pub fn update_aabb_box(mut query: Query<(&mut Aabb, &Pos, &Vel, &BoxCollider)>) {
    for (mut aabb, pos, vel, rbox) in query.iter_mut() {
        let margin = COLLISION_PAIR_VEL_MARGIN_FACTOR * vel.0.length();
        let half_extents = rbox.size / 2. + Vec2::splat(margin);
        aabb.min = pos.0 - half_extents;
        aabb.max = pos.0 + half_extents;
    }
}

pub fn collect_collision_pairs(
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
    let overlap = (half_a + half_b) - ab.abs();
    if overlap.x < 0. || overlap.y < 0. {
        None
    } else if overlap.x < overlap.y {
        Some(Contact {
            penetration: overlap.x,
            normal: Vec2::X * ab.x.signum(),
        })
    } else {
        Some(Contact {
            penetration: overlap.y,
            normal: Vec2::Y * ab.y.signum(),
        })
    }
}

pub fn integrate(
    mut query: Query<(
        &mut Pos,
        &mut PrevPos,
        &mut Vel,
        &mut PreSolveVel,
        &Mass,
        &mut Forces,
        &Drag,
    )>,
    gravity: Res<Gravity>,
) {
    for (mut pos, mut prev_pos, mut vel, mut pre_solve_vel, mass, mut forces, drag) in
        query.iter_mut()
    {
        prev_pos.0 = pos.0;

        let gravitation_force = mass.0 * gravity.0;
        let external_forces = gravitation_force;

        let forces_sum: Vec2 = forces.0.iter().chain(&[external_forces]).sum();
        forces.0.clear();

        // new velocity based on forces
        let new_velocity = vel.0 += DT * forces_sum / mass.0;

        // add drag factor
        {
            let velocity = vel.0;
            vel.0 -= velocity * velocity.abs() * drag.0;
        }

        pos.0 += DT * vel.0;

        pre_solve_vel.0 = vel.0;
    }
}

pub fn constrain_body_positions(
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

pub fn solve_pos(
    query: Query<(&mut Pos, &BoxCollider, &Mass)>,
    mut contacts: ResMut<Contacts>,
    collision_pairs: Res<CollisionPairs>,
) {
    contacts.0.clear();
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

pub fn update_vel(mut query: Query<(&Pos, &PrevPos, &mut Vel)>) {
    for (pos, prev_pos, mut vel) in query.iter_mut() {
        vel.0 = (pos.0 - prev_pos.0) / DT;
        if vel.0.x.abs() <= 0.05 {
            vel.0.x = 0.0;
        }
        if vel.0.y.abs() <= 0.05 {
            vel.0.y = 0.0;
        }
    }
}

pub fn solve_vel(
    query: Query<(&mut Vel, &PreSolveVel, &Pos, &Mass, &Restitution)>,
    contacts: Res<Contacts>,
) {
    for (entity_a, entity_b, _n) in contacts.0.iter().cloned() {
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
        transform.translation = pos.0.extend(1.);
    }
}

pub fn draw_box_collider(mut gizmos: Gizmos, query: Query<(&Pos, &BoxCollider)>) {
    for (pos, box_collider) in query.iter() {
        gizmos.rect_2d(pos.0, Rot2::IDENTITY, box_collider.size, Color::WHITE);
    }
}

pub fn draw_velocity(mut gizmos: Gizmos, query: Query<(&Pos, &Vel)>) {
    for (pos, vel) in query.iter() {
        gizmos.arrow_2d(pos.0, pos.0 + vel.0 * 0.3, Color::srgb(0., 255., 0.));
    }
}
