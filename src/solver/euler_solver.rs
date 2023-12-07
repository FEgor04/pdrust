use bevy::prelude::{Mut, Quat, Resource, Transform, Vec3};

use crate::body::RigidBody;

use super::SimulationSolver;

#[derive(Resource)]
pub struct EulerSolver {}

impl SimulationSolver for EulerSolver {
    fn step(&self, mut body: Mut<RigidBody>, mut transform: Mut<Transform>, dt: f32) {
        let position_derivative = body.get_velocity();
        transform.translation += position_derivative * dt;

        if body.angular_momentum != Vec3::ZERO {
            transform.rotation = transform.rotation.normalize();
            let omega = body.get_angular_velocity(&transform);
            let qomega = Quat::from_vec4(omega.extend(0.0));
            let qdot = qomega * transform.rotation;
            transform.rotation.x += qdot.x * dt * 0.5;
            transform.rotation.y += qdot.y * dt * 0.5;
            transform.rotation.z += qdot.z * dt * 0.5;
            transform.rotation.w += qdot.w * dt * 0.5;
            transform.rotation = transform.rotation.normalize();
        }

        let pulse_derivative = body.force;
        body.pulse += pulse_derivative * dt;

        let angular_momentum_der = body.torque * dt;
        body.angular_momentum += angular_momentum_der;
    }
}
