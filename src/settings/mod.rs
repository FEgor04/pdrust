use bevy::prelude::*;

#[derive(Resource)]
pub struct SettingsResource {
    pub method: IntergrationMethod,
    pub integration_substeps: usize,
    pub constraints_substeps: usize,
    pub baumgarte_constant: f32,
    pub gravity_vector: Vec3,
    pub slow_motion_koef: f32,
    pub print_energy_in_console: bool,
}

impl Default for SettingsResource {
    fn default() -> Self {
        Self {
            method: IntergrationMethod::EulerMethod,
            integration_substeps: 8,
            constraints_substeps: 16,
            baumgarte_constant: 0.1,
            slow_motion_koef: 1.0,
            gravity_vector: Vec3::new(0.0, -9.81, 0.0),
            print_energy_in_console: false,
        }
    }
}

pub enum IntergrationMethod {
    EulerMethod,
}
