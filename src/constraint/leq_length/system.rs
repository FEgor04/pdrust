use std::f32::INFINITY;

use bevy::prelude::*;

use crate::body::{Body, RigidBody};

use super::LowerThanOrEqualLengthConstraint;

const CONSTRAINTS_INTEGRATION_COUNT: u32 = 10;

pub fn handle_leq_length_constraints(
    constraints: Query<&LowerThanOrEqualLengthConstraint>,
    mut bodies_query: Query<
        (&mut Body, &mut Transform, Option<&mut RigidBody>),
        Without<LowerThanOrEqualLengthConstraint>,
    >,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    let constraint_dt = dt / CONSTRAINTS_INTEGRATION_COUNT as f32;
    for _ in 0..CONSTRAINTS_INTEGRATION_COUNT {
        for constraint in &constraints {
            let [(b1, t1, mut rb1), (b2, t2, mut rb2)] = bodies_query
                .get_many_mut([constraint.first_body, constraint.second_body])
                .unwrap();

            let x1 = t1.translation;
            let x2 = t2.translation;

            let v1 = rb1
                .as_ref()
                .map(|b| b.get_velocity())
                .unwrap_or_else(|| Vec3::ZERO);
            let v2 = rb2
                .as_ref()
                .map(|b| b.get_velocity())
                .unwrap_or_else(|| Vec3::ZERO);

            let m1 = rb1.as_ref().map(|b| b.mass).unwrap_or_else(|| INFINITY);
            let m2 = rb2.as_ref().map(|b| b.mass).unwrap_or_else(|| INFINITY);

            let relative_pos = x1 - x2;
            let current_length = relative_pos.length();
            let offset = constraint.length - current_length;

            if constraint.length > current_length {
                continue;
            }

            let offset_dir = relative_pos.normalize();

            let relative_velocity = v1 - v2;
            let constraint_mass = m1.powi(-1) + m2.powi(-1);

            if constraint_mass > 0.0 {
                let velocity_dot = relative_velocity.dot(offset_dir);

                let bias_factor = 0.01;
                let bias = -(bias_factor / constraint_dt) * offset;

                let lambda = -(velocity_dot + bias) / constraint_mass;

                let a_impulse = offset_dir * lambda;
                let b_impulse = -offset_dir * lambda;

                if let Some(mut body) = rb1 {
                    body.pulse += a_impulse;
                    body.angular_momentum += (x1 - t1.translation).cross(a_impulse);
                }

                if let Some(mut body) = rb2 {
                    body.pulse += b_impulse;
                    body.angular_momentum += (x2 - t2.translation).cross(a_impulse);
                }
            }
        }
    }
}
