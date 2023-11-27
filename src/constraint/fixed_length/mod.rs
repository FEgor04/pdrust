use bevy::prelude::*;

pub mod system;

#[derive(Component)]
pub struct FixedLengthConstraint {
    first_body: Entity,
    second_body: Entity,
    length: f32,
}

impl FixedLengthConstraint {
    pub fn new(first_body: Entity, second_body: Entity, length: f32) -> Self {
        return Self {
            first_body,
            second_body,
            length,
        };
    }
}
