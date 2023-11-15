use bevy::prelude::*;

use super::Body;

#[derive(Component, Clone, Copy)]
pub struct RigidBody {
    /* Constant values */
    /// Mass of a body
    pub mass: f32,
    /// Intertia tensor of a body in **Body** coordinates
    pub intertia_tensor_body: Mat3,

    /* State variables */
    /// Pulse of a body in **World** coordinates
    pub pulse: Vec3,
    /// Angular momentum of a body in **World** coordinates
    pub angular_momentum: Vec3,
    /// Total external force acting on a body in **World** coordinates
    pub force: Vec3,
    /// Total external torque action on a body in **World** coordinates
    pub torque: Vec3,

    /* Derived variables */
    /// Inversed inertia tensor of a body in **Body coordinates**
    /// Inertia_tensor_body_inv = Intertia_tensor_body^{-1}
    pub intertia_tensor_body_inv: Mat3,
}

impl Default for RigidBody {
    fn default() -> Self {
        let mass = 1.0;
        let size = 2.0;
        let inertia_tensor = Mat3::IDENTITY * (mass * size * size / 6.0);
        return Self {
            mass,
            intertia_tensor_body: inertia_tensor,
            pulse: Vec3::ZERO,
            angular_momentum: Vec3::ZERO,
            force: Vec3::ZERO,
            torque: Vec3::ZERO,
            intertia_tensor_body_inv: inertia_tensor.inverse(),
        };
    }
}

impl RigidBody {
    pub fn get_velocity(&self) -> Vec3 {
        return self.pulse / self.mass;
    }

    /// Applies a `force` to the body.
    /// Both `application_point` and `force` are given in **World** coordinates.
    pub fn apply_force(
        &mut self,
        transform: &Transform,
        application_point_world: Vec3,
        force: Vec3,
    ) {
        self.force += force;
        self.torque += (application_point_world - transform.translation).cross(force);
    }

    /// Computes keenetic energy of the body
    pub fn compute_energy(&self, transform: &Transform) -> f32 {
        let rotation_matrix = Mat3::from_quat(transform.rotation);
        let inertia_tensor_inv =
            rotation_matrix * self.intertia_tensor_body_inv * rotation_matrix.transpose();
        let omega = inertia_tensor_inv * self.angular_momentum;
        let angular_component = 0.5 * omega.dot(omega);
        self.mass * self.get_velocity().length_squared() / 2.0 + angular_component
    }

    /// Returns a velocity of a particle.
    /// `particle` is given in **World** coordinates.
    ///
    /// Total velocity of a particle is a sum of its linear velocity (which equals to body velocity)
    /// and angular velocity.
    pub fn get_particle_velocity(&self, particle: Vec3, transform: &Transform) -> Vec3 {
        self.get_particle_body_velocity(particle - transform.translation, transform)
    }

    /// Returns a velocity of a particle.
    /// `particle` is given in **Body** coordinates.
    ///
    /// Total velocity of a particle is a sum of its linear velocity (which equals to body velocity)
    /// and angular velocity.
    pub fn get_particle_body_velocity(&self, particle: Vec3, transform: &Transform) -> Vec3 {
        let rotation_matrix = Mat3::from_quat(transform.rotation);
        let inertia_tensor_inv =
            rotation_matrix * self.intertia_tensor_body_inv * rotation_matrix.transpose();
        let omega = inertia_tensor_inv * self.angular_momentum;
        self.get_velocity() + omega.cross(particle)
    }

    /// Applies a force to a body.
    /// Force is given in **World** coordinates, application_point_body is given in **Body**
    /// coordinates.
    pub fn apply_force_body_coords(
        &mut self,
        application_point_body: Vec3,
        force: Vec3,
        transform: &Transform,
    ) {
        let application_point_world =
            Body.body_to_world_coordinates(application_point_body, transform);
        self.apply_force(transform, application_point_world, force)
    }

    /// Creates a new body with given `inertia_tensor_body`, `mass`, `pulse` and `angular momentum`.
    pub fn new(mass: f32, intertia_tensor_body: Mat3, pulse: Vec3, angular_momentum: Vec3) -> Self {
        Self {
            mass,
            intertia_tensor_body,
            pulse,
            angular_momentum,
            force: Vec3::ZERO,
            torque: Vec3::ZERO,
            intertia_tensor_body_inv: intertia_tensor_body.inverse(),
        }
    }

    /// Creates a new body wih an inertia tensor of a box with given `mass`, `length`s, `pulse` and
    /// `angular_momentum`.
    pub fn new_box(
        mass: f32,
        x_length: f32,
        y_length: f32,
        z_length: f32,
        pulse: Vec3,
        angular_momentum: Vec3,
    ) -> Self {
        let inertia_tensor = Mat3::from_cols(
            Vec3::from_array([y_length.powi(2) + z_length.powi(2), 0.0, 0.0]),
            Vec3::from_array([0.0, x_length.powi(2) + z_length.powi(2), 0.0]),
            Vec3::from_array([0.0, 0.0, x_length.powi(2) + y_length.powi(2)]),
        );
        RigidBody::new(mass, inertia_tensor, pulse, angular_momentum)
    }
}
