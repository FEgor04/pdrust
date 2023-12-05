use bevy::prelude::*;

use crate::body::{Body, RigidBody};

use super::PulleyConstraint;

const CONSTRAINTS_INTEGRATION_COUNT: u32 = 16;

pub fn solve_pulley_constraints(
    constraints: Query<&PulleyConstraint>,
    mut bodies_query: Query<
        (&mut Body, &mut Transform, Option<&mut RigidBody>),
        Without<PulleyConstraint>,
    >,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    let constraint_dt = dt / CONSTRAINTS_INTEGRATION_COUNT as f32;
    for _ in 0..CONSTRAINTS_INTEGRATION_COUNT {
        for constraint in &constraints {
            let [(_b1, t1, rb1), (_b2, t2, rb2)] = bodies_query
                .get_many_mut([constraint.first_body, constraint.second_body])
                .unwrap();

            let x1 = Body.body_to_world_coordinates(constraint.first_body_offset, &t1);
            let x2 = Body.body_to_world_coordinates(constraint.second_body_offset, &t2);

            let v1 = rb1
                .as_ref()
                .map(|b| b.get_velocity())
                .unwrap_or_else(|| Vec3::ZERO);
            let v2 = rb2
                .as_ref()
                .map(|b| b.get_velocity())
                .unwrap_or_else(|| Vec3::ZERO);

            let omega1 = rb1
                .as_ref()
                .map(|b| b.get_angular_velocity(&t1))
                .unwrap_or_else(|| Vec3::ZERO);
            let omega2 = rb2
                .as_ref()
                .map(|b| b.get_angular_velocity(&t2))
                .unwrap_or_else(|| Vec3::ZERO);

            let m1_inversed = rb1.as_ref().map(|b| 1.0 / b.mass).unwrap_or_else(|| 0.0);
            let m2_inversed = rb2.as_ref().map(|b| 1.0 / b.mass).unwrap_or_else(|| 0.0);

            let i1_inversed = rb1
                .as_ref()
                .map(|b| b.get_inertia_tensor_inv(&t1))
                .unwrap_or_else(|| Mat3::ZERO);
            let i2_inversed = rb2
                .as_ref()
                .map(|b| b.get_inertia_tensor_inv(&t2))
                .unwrap_or_else(|| Mat3::ZERO);

            let r1 = x1 - t1.translation;
            let r2 = x2 - t2.translation;

            let d1 = x1 - constraint.pulley_position;
            let d2 = x2 - constraint.pulley_position;

            let target_distance = constraint.max_distance;

            let current_distance = d1.length() + d2.length();
            let distance_offset = current_distance - target_distance;
            println!("distance_offset offset is {}", distance_offset);

            let j1 = d1;
            let j2 = r1.cross(d1);
            let j3 = d2;
            let j4 = r2.cross(d2);

            let constraint_mass = m1_inversed * j1.dot(j1)
                + j2.dot(i1_inversed * j2)
                + m2_inversed * j3.dot(j3)
                + j4.dot(i2_inversed * j4);

            let jv = j1.dot(v1) + j2.dot(omega1) + j3.dot(v2) + j4.dot(omega2);

            let baumgarte_constant = 0.1;
            let b = (baumgarte_constant / constraint_dt) * distance_offset;

            let lambda = -(jv + b) / constraint_mass;

            if let Some(mut body) = rb1 {
                body.pulse += lambda * j1;
                body.angular_momentum += lambda * j2;
            }
            if let Some(mut body) = rb2 {
                body.pulse += lambda * j3;
                body.angular_momentum += lambda * j4;
            }
        }
    }
}

