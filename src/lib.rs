use bevy::prelude::*;
use energy::{
    print_total_sum_of_energy, update_energy_for_rigid_bodies, update_energy_for_springs,
};
use solver::{clean_forces_and_torque, gravity, step_in_simulation};
use springs::systems::{handle_spring_forces, update_spring_transformation};

pub mod body;
pub mod energy;
pub mod solver;
pub mod springs;

pub struct PDRustPlugin;

impl bevy::prelude::Plugin for PDRustPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                clean_forces_and_torque,
                gravity.after(clean_forces_and_torque),
                handle_spring_forces.after(gravity),
                step_in_simulation.after(handle_spring_forces),
                update_energy_for_springs.after(update_spring_transformation),
                update_energy_for_rigid_bodies.after(update_energy_for_springs),
                print_total_sum_of_energy.after(update_energy_for_rigid_bodies),
                update_spring_transformation.after(step_in_simulation),
            ),
        );
    }
}
