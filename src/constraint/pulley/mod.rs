use bevy::prelude::*;

pub mod bundle;
pub mod system;

#[derive(Component)]
pub struct PulleyConstraint {
    first_body: Entity,
    second_body: Entity,
    first_body_offset: Vec3,
    second_body_offset: Vec3,
    max_distance: f32,
}

impl PulleyConstraint {
    pub fn new(
        first_body: Entity,
        second_body: Entity,
        first_body_offset: Vec3,
        second_body_offset: Vec3,
        max_distance: f32,
    ) -> Self {
        Self {
            first_body,
            second_body,
            first_body_offset,
            second_body_offset,
            max_distance,
        }
    }
}
