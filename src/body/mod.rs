use bevy::prelude::*;

pub use rigid_body::RigidBody;

pub mod bundle;
pub mod rigid_body;

/// A marker to show that entity is actually a body
#[derive(Component, Default)]
pub struct Body;

impl Body {
    /// Converts a point in **Body** coordinates to **World** coordinates.
    pub fn body_to_world_coordinates(&self, body_coordinates: Vec3, transform: &Transform) -> Vec3 {
        return transform.translation + transform.rotation.mul_vec3(body_coordinates);
    }

    pub fn world_to_body_coordinates(
        &self,
        world_coordinates: Vec3,
        transform: &Transform,
    ) -> Vec3 {
        return transform
            .rotation
            .inverse()
            .mul_vec3(world_coordinates - transform.translation);
    }
}
