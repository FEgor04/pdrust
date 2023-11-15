use bevy::prelude::*;

use crate::energy::Energy;

use super::Spring;

#[derive(Bundle)]
pub struct SpringBundle {
    spring: Spring,
    pbr_bundle: PbrBundle,
    energy: Energy,
}

impl SpringBundle {
    pub fn new(
        first_body: Entity,
        first_body_point: Vec3,
        second_body: Entity,
        second_body_point: Vec3,
        rest_length: f32,
        spring_constant: f32,
        damping_constant: f32,
        pbr_bundle: PbrBundle,
    ) -> Self {
        Self {
            spring: Spring {
                first_body,
                first_body_attachment_point_offset: first_body_point,
                second_body,
                second_body_attachment_point_offset: second_body_point,
                rest_length,
                spring_constant,
                damping_constant,
            },
            pbr_bundle,
            energy: Energy::default(),
        }
    }
}
