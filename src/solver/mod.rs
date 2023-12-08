use crate::settings::{IntergrationMethod, SettingsResource};

use self::euler_solver::EulerSolver;
use bevy::prelude::*;

use super::body::RigidBody;

mod euler_solver;

trait SimulationSolver: Resource {
    fn step(&self, body: Mut<RigidBody>, transform: Mut<Transform>, dt: f32);
}

pub fn clean_forces_and_torque(mut query: Query<&mut RigidBody>) {
    for mut body in query.iter_mut() {
        body.force = Vec3::ZERO;
        body.torque = Vec3::ZERO;
    }
}

pub fn step_in_simulation(
    mut query: Query<(&mut RigidBody, &mut Transform)>,
    time: Res<Time>,
    settings: Res<SettingsResource>,
) {
    let solver: &dyn SimulationSolver = match settings.method {
        IntergrationMethod::EulerMethod => &EulerSolver {},
    };
    let slow_motion_koef: f32 = settings.slow_motion_koef;
    let substeps = settings.integration_substeps;
    let dt = time.delta_seconds() / slow_motion_koef as f32 / (substeps as f32);
    for _step in 0..substeps {
        for (body, transform) in query.iter_mut() {
            solver.step(body, transform, dt);
        }
    }
}

pub fn gravity(mut query: Query<(&mut RigidBody, &Transform)>, settings: Res<SettingsResource>) {
    let gravity_acc = settings.gravity_vector;
    for (mut body, transform) in query.iter_mut() {
        let gravity = gravity_acc * body.mass;
        body.apply_force_body_coords(Vec3::ZERO, gravity, transform)
    }
}
