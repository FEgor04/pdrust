/// This example shows a double pendulum made with springs.
/// Try to change body attachment point to see what happens!
mod utils;
use bevy::{
    prelude::*,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use pdrust::{
    body::{bundle::RigidBodyBundle, Body},
    springs::bundle::SpringBundle,
};
use utils::ExamplesUtilsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(pdrust::PDRustPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_plugins(ExamplesUtilsPlugin)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let b1 = commands
    //     .spawn(RigidBodyBundle::new_box(
    //         PbrBundle {
    //             mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
    //             material: materials.add(Color::RED.into()),
    //             transform: Transform::from_xyz(5.0, 0.0, 5.0),
    //             ..default()
    //         },
    //         1.0,
    //         1.0,
    //         1.0,
    //         1.0,
    //         Vec3::ZERO,
    //         Vec3::ZERO,
    //     ))
    //     .id();

    let b1 = RigidBodyBundle::spawn_new_box(
        &mut commands,
        &mut meshes,
        materials.add(Color::RED.into()),
        1.0,
        1.0,
        1.0,
        1.0,
        Transform::from_xyz(5.0, 0.0, 5.0),
        Vec3::ZERO,
        Vec3::ZERO,
    );

    let b2 = RigidBodyBundle::spawn_new_box(
        &mut commands,
        &mut meshes,
        materials.add(Color::RED.into()),
        1.0,
        2.0,
        1.0,
        2.0,
        Transform::from_xyz(5.0, 0.0, 0.0),
        Vec3::ZERO,
        Vec3::ZERO,
    );

    let anchor = commands
        .spawn((Transform::from_xyz(0.0, 5.0, 0.0), Body))
        .id();

    let _spring1 = commands
        .spawn(SpringBundle::new(
            b1,
            Vec3::new(0.5, 0.5, 0.5),
            b2,
            Vec3::new(0.0, -0.5, 0.0),
            5.0,
            5.0,
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
        ))
        .id();

    let _spring2 = commands
        .spawn(SpringBundle::new(
            b2,
            Vec3::new(0.0, 0.5, 0.0),
            anchor,
            Vec3::new(0.0, 0.0, 0.0),
            5.0,
            5.0,
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
