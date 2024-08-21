use bevy::{
    ecs::query::{QueryData, QueryEntityError, QueryFilter, WorldQuery},
    prelude::*,
};

pub trait QueryExt<'a, Q: WorldQuery> {
    /// Get mutable access to the components of a pair entities in this query
    fn get_pair_mut(
        &mut self,
        a: Entity,
        b: Entity,
    ) -> Result<(Q::Item<'a>, Q::Item<'a>), QueryEntityError>;
}

impl<'a, 'w, 's, Q: WorldQuery, D: QueryData, F: QueryFilter> QueryExt<'a, Q>
    for Query<'w, 's, D, F>
{
    fn get_pair_mut(
        &mut self,
        a: Entity,
        b: Entity,
    ) -> Result<(D::Item<'a>, D::Item<'a>), QueryEntityError> {
        let (res_a, res_b) = unsafe {
            // Ensure safety
            assert!(a != b);
            (self.get_unchecked(a), self.get_unchecked(b))
        };
        match (res_a, res_b) {
            (Ok(res_a), Ok(res_b)) => Ok((res_a, res_b)),
            _ => Err(QueryEntityError::NoSuchEntity(a)),
        }
    }
}
