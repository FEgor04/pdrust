use bevy::prelude::*;

use crate::energy::Energy;

use super::{Body,rigid_body::RigidBody};

#[derive(Bundle, Default)]
pub struct RigidBodyBundle {
    pbr_bundle: PbrBundle,
    body: Body,
    rigid_body: RigidBody,
    energy: Energy,
}

impl RigidBodyBundle {
    /// Creates a new rigid body box.
    pub fn new_box(
        pbr_bundle: PbrBundle,
        mass: f32,
        x_length: f32,
        y_length: f32,
        z_length: f32,
        pulse: Vec3,
        angular_momentum: Vec3,
    ) -> Self {
        Self {
            pbr_bundle,
            body: Body,
            rigid_body: RigidBody::new_box(
                mass,
                x_length,
                y_length,
                z_length,
                pulse,
                angular_momentum,
            ),
            energy: Energy::default(),
        }
    }
}
