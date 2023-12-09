use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use pdrust::{
    body::{bundle::RigidBodyBundle, Body},
    constraint::distance::bundle::DistanceConstraintBundle, settings::SettingsResource,
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
    mut settings: ResMut<SettingsResource>,
) {
    settings.print_energy_in_console = true;

    let start = Vec3::new(-5.0, 0.0, 0.0);
    let cube_size = 0.2;
    let n = 50;
    let constrait_size = 1.0;

    let mut bodies = vec![];

    bodies.push(
        commands
            .spawn((
                Body,
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::UVSphere {
                        radius: 0.1,
                        ..default()
                    })),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_translation(start),
                    ..default()
                },
            ))
            .id(),
    );

    for i in 0..n {
        let mut pos = start;
        pos.x += constrait_size * (i as f32 + 1.0) + cube_size * i as f32;

        bodies.push(RigidBodyBundle::spawn_new_box(
            &mut commands,
            &mut meshes,
            materials.add(Color::GREEN.into()),
            1.0,
            cube_size,
            cube_size,
            cube_size,
            Transform::from_translation(pos),
            Vec3::ZERO,
            Vec3::ZERO,
        ))
    }

    let mut final_pos = start;
    final_pos.x += constrait_size * (n as f32 + 1.0) + cube_size * n as f32;
    bodies.push(
        commands
            .spawn((
                Body,
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::UVSphere {
                        radius: 0.2,
                        ..default()
                    })),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_translation(final_pos),
                    ..default()
                },
            ))
            .id(),
    );

    for i in 0..bodies.len() - 1 {
        DistanceConstraintBundle::spawn_new(
            &mut commands,
            &mut meshes,
            materials.add(Color::AZURE.into()),
            bodies[i],
            bodies[i + 1],
            Vec3::ZERO,
            Vec3::ZERO,
            constrait_size,
            constrait_size,
        );
    }

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
