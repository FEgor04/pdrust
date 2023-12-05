use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use pdrust::{
    body::bundle::RigidBodyBundle,
    constraint::pulley::bundle::PulleyBundle,
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
    let l = 10.0;
    let m1 = 10.0;
    let m2 = m1 * 2.0;
    let m_central = m1 + m2;

    let equilibrium_pos = Vec3::new(0.0, -l / 3_f32.powf(0.5), 0.0);
    let equilibrium_offset: f32 = 0.0;
    let b_central_pos = equilibrium_pos + Vec3::new(0.0, equilibrium_offset, 0.0);

    let pulley1_pos = Vec3::new(-l, 0.0, 0.0);
    let pulley2_pos = Vec3::new(l, 0.0, 0.0);


    let constraint_distance = 3.0 * l;
    let vertical_offset = constraint_distance - (b_central_pos - pulley1_pos).length();
    let b1_pos = pulley1_pos + Vec3::new(0.0, -vertical_offset, 0.0);
    let b2_pos = pulley2_pos + Vec3::new(0.0, -vertical_offset, 0.0);

    let b1 = RigidBodyBundle::spawn_new_box(
        &mut commands,
        &mut meshes,
        materials.add(Color::RED.into()),
        m1,
        1.0,
        1.0,
        1.0,
        Transform::from_translation(b1_pos),
        Vec3::ZERO,
        Vec3::ZERO,
    );

    let b2 = RigidBodyBundle::spawn_new_box(
        &mut commands,
        &mut meshes,
        materials.add(Color::RED.into()),
        m2,
        1.0,
        1.0,
        1.0,
        Transform::from_translation(b2_pos),
        Vec3::ZERO,
        Vec3::ZERO,
    );

    let central_body = RigidBodyBundle::spawn_new_sphere(
        &mut commands,
        &mut meshes,
        materials.add(Color::GREEN.into()),
        m_central,
        0.5,
        Transform::from_translation(b_central_pos),
        Vec3::ZERO,
        Vec3::ZERO,
    );

    PulleyBundle::spawn_new(
        &mut commands,
        &mut meshes,
        materials.add(Color::MIDNIGHT_BLUE.into()),
        materials.add(Color::MIDNIGHT_BLUE.into()),
        materials.add(Color::BEIGE.into()),
        b1,
        central_body,
        Vec3::ZERO,
        Vec3::ZERO,
        constraint_distance,
        pulley1_pos,
    );

    PulleyBundle::spawn_new(
        &mut commands,
        &mut meshes,
        materials.add(Color::MIDNIGHT_BLUE.into()),
        materials.add(Color::MIDNIGHT_BLUE.into()),
        materials.add(Color::BEIGE.into()),
        b2,
        central_body,
        Vec3::ZERO,
        Vec3::ZERO,
        constraint_distance,
        pulley2_pos,
    );

    let pulley_radius = 0.25;

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: pulley_radius,
            ..default()
        })),
        material: materials.add(Color::CYAN.into()),
        transform: Transform::from_translation(equilibrium_pos),
        ..default()
    });

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
