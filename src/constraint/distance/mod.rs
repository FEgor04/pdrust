use bevy::prelude::*;

pub mod bundle;
pub mod system;

#[derive(Component)]
pub struct DistanceConstraint {
    first_body: Entity,
    second_body: Entity,
    first_body_offset: Vec3,
    second_body_offset: Vec3,
    min_distance: f32,
    max_distance: f32,
}

impl DistanceConstraint {
    pub fn new(
        first_body: Entity,
        second_body: Entity,
        first_body_offset: Vec3,
        second_body_offset: Vec3,
        min_distance: f32,
        max_distance: f32,
    ) -> Self {
        return Self {
            first_body,
            second_body,
            first_body_offset,
            second_body_offset,
            min_distance,
            max_distance,
        };
    }
}
