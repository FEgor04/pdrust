use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use pdrust::{
    body::{bundle::RigidBodyBundle, Body, RigidBody},
    constraint::fixed_length::FixedLengthConstraint,
    springs::{bundle::SpringBundle, Spring},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(pdrust::PDRustPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let l1 = 3.0;
    let l2 = 2.0;
    let r = 0.25;

    // cube
    let b1 = commands
        .spawn(RigidBodyBundle::new_box(
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: r,
                    ..default()
                })),
                material: materials.add(Color::RED.into()),
                transform: Transform::from_xyz(l1, 0.0, 0.0),
                ..default()
            },
            1.0,
            1.0,
            1.0,
            1.0,
            Vec3::ZERO,
            Vec3::ZERO,
        ))
        .id();

    // cube
    let b2 = commands
        .spawn(RigidBodyBundle::new_box(
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: r,
                    ..default()
                })),
                material: materials.add(Color::RED.into()),
                transform: Transform::from_xyz(l1 - l2, 0.0, 0.0),
                ..default()
            },
            1.0,
            1.0,
            1.0,
            1.0,
            Vec3::ZERO,
            Vec3::ZERO,
        ))
        .id();

    let anchor = commands
        .spawn((
            Body,
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.1,
                    ..default()
                })),
                material: materials.add(Color::RED.into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
        ))
        .id();

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

    commands.spawn(FixedLengthConstraint::new(b1, anchor, l1, l1));

    commands.spawn(FixedLengthConstraint::new(b1, b2, l2, l2));

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
