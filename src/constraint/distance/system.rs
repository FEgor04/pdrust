use bevy::prelude::*;

use crate::body::{Body, RigidBody};

use super::DistanceConstraint;

const CONSTRAINTS_INTEGRATION_COUNT: u32 = 10;

pub fn solve_distance_constraints(
    constraints: Query<&DistanceConstraint>,
    mut bodies_query: Query<
        (&mut Body, &mut Transform, Option<&mut RigidBody>),
        Without<DistanceConstraint>,
    >,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    let _constraint_dt = dt / CONSTRAINTS_INTEGRATION_COUNT as f32;
    for _ in 0..CONSTRAINTS_INTEGRATION_COUNT {
        for constraint in &constraints {
            let [(_b1, t1, rb1), (_b2, t2, rb2)] = bodies_query
                .get_many_mut([constraint.first_body, constraint.second_body])
                .unwrap();

            let x1 = Body.body_to_world_coordinates(constraint.first_body_offset, &t1);
            let x2 = Body.body_to_world_coordinates(constraint.second_body_offset, &t2);

            let v1 = rb1
                .as_ref()
                .map(|b| b.get_particle_body_velocity(constraint.first_body_offset, &t1))
                .unwrap_or_else(|| Vec3::ZERO);
            let v2 = rb2
                .as_ref()
                .map(|b| b.get_particle_body_velocity(constraint.second_body_offset, &t2))
                .unwrap_or_else(|| Vec3::ZERO);

            let omega1 = rb1
                .as_ref()
                .map(|b| b.get_angular_velocity(&t1))
                .unwrap_or_else(|| Vec3::ZERO);
            let omega2 = rb2
                .as_ref()
                .map(|b| b.get_angular_velocity(&t2))
                .unwrap_or_else(|| Vec3::ZERO);

            let b = 0.25;

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

            let ab = x2 - x1;
            let abn = ab.normalize();

            let current_length = ab.length();
            if constraint.min_length <= current_length && current_length <= constraint.max_length {
                continue;
            }
            println!(
                "current length is {}. max_length is {}",
                current_length, constraint.max_length
            );

            let j1 = -abn;
            let j2 = -r1.cross(abn);
            let j3 = abn;
            let j4 = r2.cross(abn);

            let constraint_mass = m1_inversed * j1.length_squared()
                + j2.dot(i1_inversed * j2)
                + m2_inversed * j3.length_squared()
                + j4.dot(i2_inversed * j4);

            let jv = j1.dot(v1) + j2.dot(omega1) + j3.dot(v2) + j4.dot(omega2);

            let denom = -(jv + b);
            let lambda = denom / constraint_mass;

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

pub fn update_distance_constraints_transformation(
    mut constraints: Query<(&DistanceConstraint, &mut Transform)>,
    mut bodies_query: Query<
        (&mut Body, &mut Transform, Option<&mut RigidBody>),
        Without<DistanceConstraint>,
    >,
) {
    for (constraint, mut transform) in &mut constraints {
        let [(_b1, t1, _rb1), (_b2, t2, _rb2)] = bodies_query
            .get_many_mut([constraint.first_body, constraint.second_body])
            .unwrap();

        let x1 = Body.body_to_world_coordinates(constraint.first_body_offset, &t1);
        let x2 = Body.body_to_world_coordinates(constraint.second_body_offset, &t2);

        let constraint_length_vec = (x1) - (x2);
        let constarint_mid = (x1 + x2) / 2.0;

        // Cylinder diameter
        // Spring is rendred as a cylinder,
        transform.scale.y = constraint_length_vec.length();
        transform.scale.x = 1.0;
        transform.scale.z = transform.scale.x;
        // translation is a position of cylinder center
        transform.translation = constarint_mid;
        transform.rotation = Quat::from_rotation_arc(Vec3::Y, constraint_length_vec.normalize());
    }
}
