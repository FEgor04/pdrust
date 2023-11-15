use bevy::prelude::*;

pub mod bundle;
pub mod systems;

/// A Spring component.
/// It is considered that spring is attached right to the center of a body.
#[derive(Component)]
pub struct Spring {
    /// First body that spring is attached to
    pub first_body: Entity,
    /// Offset of a point where spring is attached relative to center of mass
    pub first_body_attachment_point_offset: Vec3,
    /// Second body that spring is attached to
    pub second_body: Entity,
    /// Offset of a point where spring is attached relative to center of mass
    pub second_body_attachment_point_offset: Vec3,
    /// Spring length in the state of rest
    pub rest_length: f32,
    /// Spring constant of a spring
    pub spring_constant: f32,
    /// Damping constant of a spring
    pub damping_constant: f32,
}
