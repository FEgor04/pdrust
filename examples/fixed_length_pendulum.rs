mod utils;

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use pdrust::{
    body::{bundle::RigidBodyBundle, Body},
    constraint::distance::bundle::DistanceConstraintBundle,
    settings::SettingsResource,
};
use utils::ExamplesUtilsPlugin;

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
    mut settings: ResMut<SettingsResource>,
) {
    settings.constraints_substeps = 32;
    settings.baumgarte_constant = 0.01;
    settings.integration_substeps = 32;
    settings.print_energy_in_console = true;

    let l1 = 5.0;
    let l2 = 5.0;

    let b1 = RigidBodyBundle::spawn_new_box(
        &mut commands,
        &mut meshes,
        materials.add(Color::RED.into()),
        1.0,
        1.0,
        1.0,
        1.0,
        Transform::from_xyz(l1, 0.0, 0.0),
        Vec3::ZERO,
        Vec3::ZERO,
    );

    let b2 = RigidBodyBundle::spawn_new_box(
        &mut commands,
        &mut meshes,
        materials.add(Color::GOLD.into()),
        1.0,
        1.0,
        1.0,
        1.0,
        Transform::from_xyz(l1 + l2, 0.0, 0.0),
        Vec3::ZERO,
        Vec3::ZERO,
    );

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

    let _c1 = DistanceConstraintBundle::spawn_new(
        &mut commands,
        &mut meshes,
        materials.add(Color::AZURE.into()),
        anchor,
        b1,
        Vec3::ZERO,
        Vec3::ZERO,
        l1,
        l1,
    );

    let _c2 = DistanceConstraintBundle::spawn_new(
        &mut commands,
        &mut meshes,
        materials.add(Color::AZURE.into()),
        b1,
        b2,
        Vec3::ZERO,
        Vec3::ZERO,
        l2,
        l2,
    );

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
