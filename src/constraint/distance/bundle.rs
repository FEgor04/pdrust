use bevy::prelude::*;

use super::DistanceConstraint;

#[derive(Bundle)]
pub struct DistanceConstraintBundle {
    constraint: DistanceConstraint,
    pbr_bundle: PbrBundle,
}

impl DistanceConstraintBundle {
    pub fn spawn_new(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<StandardMaterial>,
        first_body: Entity,
        second_body: Entity,
        first_body_offset: Vec3,
        second_body_offset: Vec3,
        min_distance: f32,
        max_distance: f32,
    ) -> Entity {
        commands
            .spawn(DistanceConstraintBundle {
                constraint: DistanceConstraint {
                    first_body,
                    second_body,
                    first_body_offset,
                    second_body_offset,
                    min_distance,
                    max_distance,
                },
                pbr_bundle: PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cylinder {
                        height: 1.0,
                        radius: 0.05,
                        ..default()
                    })),
                    material,
                    ..default()
                },
            })
            .id()
    }
}
