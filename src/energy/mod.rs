use bevy::prelude::*;

use crate::{
    body::{Body, RigidBody},
    springs::Spring,
};

/// Something that has an energy that can be computed.
#[derive(Component)]
pub struct Energy {
    energy: f32,
}

impl Default for Energy {
    fn default() -> Self {
        return Self { energy: 0.0 };
    }
}

pub fn update_energy_for_springs(
    mut springs_query: Query<(&Spring, &mut Energy)>,
    bodies_query: Query<(Entity, &Body, &Transform), Without<Spring>>,
) {
    for (spring, mut spring_energy) in springs_query.iter_mut() {
        let [(_, b1, t1), (_, b2, t2)] = bodies_query
            .get_many([spring.first_body, spring.second_body])
            .unwrap();
        let sp1 = b1.body_to_world_coordinates(spring.first_body_attachment_point_offset, t1);
        let sp2 = b2.body_to_world_coordinates(spring.second_body_attachment_point_offset, t2);
        let spring_length_vec = sp2 - sp1;
        let spring_length = spring_length_vec.length();
        let delta_length = spring_length - spring.rest_length;

        // spring_energy.energy = spring.spring_constant * delta_length.abs() * delta_length / 2.0;
        spring_energy.energy = spring.spring_constant * delta_length.powi(2) / 2.0;
    }
}

pub fn update_energy_for_rigid_bodies(
    mut bodies_query: Query<(&RigidBody, &Body, &Transform, &mut Energy)>,
) {
    for (rb, _b, t, mut e) in bodies_query.iter_mut() {
        e.energy = rb.compute_energy(t);
    }
}

pub fn print_total_sum_of_energy(energy_query: Query<&Energy>) {
    let sum: f32 = energy_query.iter().map(|f| f.energy).sum();
}
