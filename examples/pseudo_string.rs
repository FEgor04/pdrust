/// This example shows a double pendulum made with springs.
/// Try to change body attachment point to see what happens!
mod utils;
use utils::ExamplesUtilsPlugin;
use bevy::{
    prelude::*,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use pdrust::{
    body::{bundle::RigidBodyBundle, Body, RigidBody},
    springs::bundle::SpringBundle,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(pdrust::PDRustPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(ExamplesUtilsPlugin)
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut bodies: Vec<Entity> = vec![];
    let anchor = commands
        .spawn((Body, Transform::from_xyz(0.0, 0.0, 0.0)))
        .id();
    bodies.push(anchor);

    let n = 10;
    let final_body_point = Vec3::new(2.0, 0.0, 0.0);
    let body_attachment_point = Vec3::new(0.5, 0.5, 0.0);
    let pseudo_body_offset = (final_body_point - body_attachment_point) / (n as f32 - 1.0);
    let dx = pseudo_body_offset.length();
    println!("dx is {}", dx);

    for i in 0..n {
        let body_pos = pseudo_body_offset * (i + 1) as f32;
        let new_body = commands
            .spawn((
                Body,
                RigidBody::new_box(0.01, 0.01, 0.01, 0.01, Vec3::ZERO, Vec3::ZERO),
                Transform::from_translation(body_pos),
            ))
            .id();
        bodies.push(new_body);
    }

    bodies.push(RigidBodyBundle::spawn_new_box(
        &mut commands,
        &mut meshes,
        materials.add(Color::RED.into()),
        1.0,
        1.0,
        1.0,
        1.0,
        Transform::from_translation(final_body_point),
        Vec3::ZERO,
        Vec3::ZERO,
    ));

    for i in 1..bodies.len() - 1 {
        commands.spawn(SpringBundle::new(
            bodies[i - 1],
            Vec3::ZERO,
            bodies[i],
            Vec3::ZERO,
            dx,
            50.0,
            1.0,
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cylinder {
                    radius: 0.1,
                    height: 1.0,
                    ..Default::default()
                })),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                material: materials.add(Color::BLUE.into()),
                ..default()
            },
        ));
    }

    commands.spawn(SpringBundle::new(
        bodies[bodies.len() - 2],
        Vec3::ZERO,
        bodies[bodies.len() - 1],
        body_attachment_point,
        dx,
        10.0 * (n as f32),
        1.0,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 0.1,
                height: 1.0,
                ..Default::default()
            })),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(Color::BLUE.into()),
            ..default()
        },
    ));

    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 20.0, 15.0)
                .looking_at(Vec3::from_array([0.0, 10.0, 0.0]), Vec3::Y),
            ..default()
        },
        PanOrbitCamera {
            focus: Vec3::from_array([0.0, 0.0, 0.0]),
            ..default()
        },
    ));
}
