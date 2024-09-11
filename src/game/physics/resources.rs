use bevy::prelude::*;

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
