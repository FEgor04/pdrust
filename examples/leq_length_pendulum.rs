mod utils;
use utils::ExamplesUtilsPlugin;
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use pdrust::{
    body::{bundle::RigidBodyBundle, Body},
    constraint::distance::bundle::DistanceConstraintBundle,
    settings::SettingsResource,
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
    mut settings: ResMut<SettingsResource>,
) {
    settings.print_energy_in_console = true;
    settings.constraints_substeps = 32;
    settings.integration_substeps = 32;
    settings.baumgarte_constant = 0.05;

    let l1 = 3.0;
    let l2 = 2.0;
    let r1 = 0.25;
    let m1 = 1.0;
    let m2 = 1.0;
    let r2 = r1 * m2 / m1;

    let b1 = RigidBodyBundle::spawn_new_sphere(
        &mut commands,
        &mut meshes,
        materials.add(Color::RED.into()),
        m1,
        r1,
        Transform::from_xyz(l1, 0.0, 0.0),
        Vec3::ZERO,
        Vec3::ZERO,
    );

    let b2 = RigidBodyBundle::spawn_new_sphere(
        &mut commands,
        &mut meshes,
        materials.add(Color::RED.into()),
        m2,
        r2,
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
        0.0,
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
        0.0,
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
