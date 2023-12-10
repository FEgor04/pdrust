use bevy::prelude::*;
use constraint::{
    distance::system::{solve_distance_constraints, update_distance_constraints_transformation},
    pulley::system::{solve_pulley_constraints, update_pulley_constraints_transformation},
};
use energy::{update_energy_for_rigid_bodies, update_energy_for_springs};
use settings::SettingsResource;
use solver::{clean_forces_and_torque, gravity, step_in_simulation};
use springs::systems::{handle_spring_forces, update_spring_transformation};

pub mod body;
pub mod constraint;
pub mod energy;
pub mod settings;
pub mod solver;
pub mod springs;

pub struct PDRustPlugin;

impl bevy::prelude::Plugin for PDRustPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            FixedUpdate,
            (
                clean_forces_and_torque,
                gravity.after(clean_forces_and_torque),
                handle_spring_forces.after(gravity),
                solve_distance_constraints.after(handle_spring_forces),
                solve_pulley_constraints.after(solve_distance_constraints),
                step_in_simulation.after(solve_pulley_constraints),
                update_energy_for_springs.after(update_spring_transformation),
                update_energy_for_rigid_bodies.after(update_energy_for_springs),
                update_spring_transformation.after(step_in_simulation),
                update_distance_constraints_transformation.after(step_in_simulation),
                update_pulley_constraints_transformation.after(step_in_simulation),
            ),
        )
        .insert_resource(SettingsResource::default());
    }
}
