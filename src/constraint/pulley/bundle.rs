use bevy::prelude::*;

use super::PulleyConstraint;

#[derive(Bundle)]
pub struct PulleyBundle {
    pulley: PulleyConstraint,
    render: PulleyRender,
}

#[derive(Component)]
pub struct PulleyRender {
    /// must be a pbr bundle!!!
    pub first_thread: Entity,
    pub second_thread: Entity,
}

impl PulleyBundle {
    fn new(
        first_body: Entity,
        second_body: Entity,
        first_body_offset: Vec3,
        second_body_offset: Vec3,
        max_distance: f32,
        pulley_position: Vec3,
        render: PulleyRender,
    ) -> Self {
        Self {
            pulley: PulleyConstraint {
                first_body,
                second_body,
                first_body_offset,
                second_body_offset,
                max_distance,
                pulley_position,
            },
            render,
        }
    }

    pub fn spawn_new(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        m1: Handle<StandardMaterial>,
        m2: Handle<StandardMaterial>,
        first_body: Entity,
        second_body: Entity,
        first_body_offset: Vec3,
        second_body_offset: Vec3,
        max_distance: f32,
        pulley_position: Vec3,
    ) -> Entity {
        let thread_radius = 0.1;
        let first_thread_id = commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cylinder {
                    radius: thread_radius,
                    height: 1.0,
                    ..default()
                })),
                material: m1,
                ..default()
            })
            .id();
        let second_thread_id = commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cylinder {
                    radius: thread_radius,
                    height: 1.0,
                    ..default()
                })),
                material: m2,
                ..default()
            })
            .id();
        commands
            .spawn(PulleyBundle::new(
                first_body,
                second_body,
                first_body_offset,
                second_body_offset,
                max_distance,
                pulley_position,
                PulleyRender {
                    first_thread: first_thread_id,
                    second_thread: second_thread_id,
                },
            ))
            .id()
    }
}
