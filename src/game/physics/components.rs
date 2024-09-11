use bevy::prelude::*;

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
            prev_pos: PrevPos(pos - vel),
            vel: Vel(vel),
            ..Default::default()
        }
    }
}
