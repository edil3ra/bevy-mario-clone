use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::config::TILE_SIZE;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Pos(pub Vec2);

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct PrevPos(pub Vec2);

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Mass(pub f32);
impl Default for Mass {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct PreSolveVel(pub Vec2);

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Vel(pub Vec2);

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Restitution(pub f32);

impl Default for Restitution {
    fn default() -> Self {
        Self(0.3)
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Forces(pub Vec<Vec2>);

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Drag(pub Vec2);

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Aabb {
    pub min: Vec2,
    pub max: Vec2,
}

impl Aabb {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn from_vec_size(point: Vec2, size: Vec2) -> Self {
        Self {
            min: Vec2::new(point.x, point.y),
            max: Vec2::new(point.x + size.x, point.y + size.y),
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.max.x >= other.min.x
            && self.max.y >= other.min.y
            && self.min.x <= other.max.x
            && self.min.y <= other.max.y
    }

    pub fn right(&self) -> f32 {
        self.max.x
    }
    pub fn top(&self) -> f32 {
        self.max.y
    }
    pub fn left(&self) -> f32 {
        self.min.x
    }
    pub fn bottom(&self) -> f32 {
        self.min.y
    }
}

impl From<TilePos> for Aabb {
    fn from(tile_pos: TilePos) -> Self {
        let x = (tile_pos.x * TILE_SIZE) as f32;
        let y = (tile_pos.y * TILE_SIZE) as f32;
        Self::new(
            Vec2::new(x, y),
            Vec2::new(x + TILE_SIZE as f32, y + TILE_SIZE as f32),
        )
    }
}

pub struct Contact {
    pub penetration: f32,
    pub normal: Vec2,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
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
    pub forces: Forces,
    pub drag: Drag,
}
