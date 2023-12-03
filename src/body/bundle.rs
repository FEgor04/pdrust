use bevy::prelude::*;

use crate::energy::Energy;

use super::{rigid_body::RigidBody, Body};

#[derive(Bundle, Default)]
pub struct RigidBodyBundle {
    pbr_bundle: PbrBundle,
    body: Body,
    rigid_body: RigidBody,
    energy: Energy,
}

impl RigidBodyBundle {
    /// Creates a new rigid body box.
    fn new_box(
        pbr_bundle: PbrBundle,
        mass: f32,
        x_length: f32,
        y_length: f32,
        z_length: f32,
        pulse: Vec3,
        angular_momentum: Vec3,
    ) -> Self {
        Self {
            pbr_bundle,
            body: Body,
            rigid_body: RigidBody::new_box(
                mass,
                x_length,
                y_length,
                z_length,
                pulse,
                angular_momentum,
            ),
            energy: Energy::default(),
        }
    }

    fn new_sphere(
        pbr_bundle: PbrBundle,
        mass: f32,
        r: f32,
        pulse: Vec3,
        angular_momentum: Vec3,
    ) -> Self {
        Self {
            pbr_bundle,
            body: Body,
            rigid_body: RigidBody::new_sphere(mass, r, pulse, angular_momentum),
            energy: Energy::default(),
        }
    }

    pub fn spawn_new_box(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<StandardMaterial>,
        mass: f32,
        x_length: f32,
        y_length: f32,
        z_length: f32,
        transform: Transform,
        pulse: Vec3,
        angular_momentum: Vec3,
    ) -> Entity {
        commands
            .spawn(Self::new_box(
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(x_length, y_length, z_length))),
                    material,
                    transform,
                    ..default()
                },
                mass,
                x_length,
                y_length,
                z_length,
                pulse,
                angular_momentum,
            ))
            .id()
    }

    pub fn spawn_new_sphere(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<StandardMaterial>,
        mass: f32,
        r: f32,
        transform: Transform,
        pulse: Vec3,
        angular_momentum: Vec3,
    ) -> Entity {
        commands
            .spawn(Self::new_sphere(
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::UVSphere {
                        radius: r,
                        ..default()
                    })),
                    material,
                    transform,
                    ..default()
                },
                mass,
                r,
                pulse,
                angular_momentum,
            ))
            .id()
    }
}
