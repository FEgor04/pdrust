use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use pdrust::{energy::Energy, settings::SettingsResource};

pub struct ExamplesUtilsPlugin;

impl Plugin for ExamplesUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egui::EguiPlugin);
        app.add_systems(Update, simulation_settings_ui);
    }
}

fn simulation_settings_ui(
    mut contexts: EguiContexts,
    mut settings: ResMut<SettingsResource>,
    energy: Query<&Energy>,
) {
    egui::Window::new("Simulation Settings").show(contexts.ctx_mut(), |ui| {
        ui.add(
            egui::Slider::new(&mut settings.integration_substeps, 1..=32)
                .text("Integration substeps"),
        );
        ui.add(
            egui::Slider::new(&mut settings.constraints_substeps, 1..=32)
                .text("Constraints integration substeps"),
        );
        ui.add(
            egui::Slider::new(&mut settings.baumgarte_constant, 0.0..=0.1)
                .text("Baumgarte constant"),
        );
        ui.add(
            egui::Slider::new(&mut settings.slow_motion_koef, 1.0..=16.0)
                .text("Slow Motion coefficient"),
        );
        ui.add(egui::Label::new(format!(
            "Sum of energies: {:.5}",
            energy.iter().map(|e| e.get_energy()).sum::<f32>()
        )));
    });
}
