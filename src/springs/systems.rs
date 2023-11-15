use bevy::prelude::*;

use crate::body::{Body, RigidBody};
use crate::springs::Spring;

/// Handles all the forces caused by spring (e.g. Hooke's law forces)
pub fn handle_spring_forces(
    mut springs_query: Query<&Spring>,
    mut bodies_query: Query<
        (Entity, &mut Body, &mut Transform, Option<&mut RigidBody>),
        Without<Spring>,
    >,
) {
    for spring in springs_query.iter_mut() {
        let [(_, _b1, t1, rb1), (_, _b2, t2, rb2)] = bodies_query
            .get_many_mut([spring.first_body, spring.second_body])
            .unwrap();

        let v1 = if let Some(ref rigid_body) = rb1 {
            rigid_body.get_particle_body_velocity(spring.first_body_attachment_point_offset, &t1)
        } else {
            Vec3::ZERO
        };

        let v2 = if let Some(ref rigid_body) = rb2 {
            rigid_body.get_particle_body_velocity(spring.second_body_attachment_point_offset, &t2)
        } else {
            Vec3::ZERO
        };

        let spring_velocity = v1 - v2;

        let sp1 = Body.body_to_world_coordinates(spring.first_body_attachment_point_offset, &t1);
        let sp2 = Body.body_to_world_coordinates(spring.second_body_attachment_point_offset, &t2);

        let spring_length_vec = (sp1) - (sp2);
        let spring_length = spring_length_vec.length();
        let spring_length_norm = spring_length_vec.normalize();

        let force = -(spring.spring_constant * (spring_length - spring.rest_length)
            + spring.damping_constant * spring_velocity * spring_length_norm)
            * spring_length_norm;

        if let Some(mut body) = rb1 {
            body.apply_force_body_coords(spring.first_body_attachment_point_offset, force, &t1);
        }
        if let Some(mut body) = rb2 {
            body.apply_force_body_coords(spring.second_body_attachment_point_offset, -force, &t2);
        }
    }
}

/// Updates the spring transformation.
/// It is needed to properly display spring on a screen
pub fn update_spring_transformation(
    mut springs_query: Query<(&Spring, &mut Transform)>,
    bodies_query: Query<(Entity, &Body, &Transform), Without<Spring>>,
) {
    for (spring, mut spring_transform) in springs_query.iter_mut() {
        let [(_, _b1, t1), (_, _b2, t2)] = bodies_query
            .get_many([spring.first_body, spring.second_body])
            .unwrap();

        let sp1 = Body.body_to_world_coordinates(spring.first_body_attachment_point_offset, &t1);
        let sp2 = Body.body_to_world_coordinates(spring.second_body_attachment_point_offset, &t2);

        let spring_length_vec = (sp1) - (sp2);
        let spring_mid = (sp1 + sp2) / 2.0;

        // Cylinder diameter
        let spring_koef = spring.rest_length / spring_length_vec.length();

        // Spring is rendred as a cylinder,
        spring_transform.scale.y = spring_length_vec.length();
        spring_transform.scale.x = spring_koef * 0.5;
        spring_transform.scale.z = spring_transform.scale.x;
        // translation is a position of cylinder center
        spring_transform.translation = spring_mid;
        spring_transform.rotation = Quat::from_rotation_arc(Vec3::Y, spring_length_vec.normalize());
    }
}
