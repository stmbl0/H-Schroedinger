pub mod graph2d;
pub mod modes;

use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam;

use crate::modes::ViewMode;

fn main() {
    App::new()
        // Plugins //
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_pancam::PanCamPlugin)
        .add_plugins(EguiPlugin::default())
        // Default color //
        .insert_resource(ClearColor(Color::srgb(1., 1., 1.)))
        // UI //
        .add_systems(EguiPrimaryContextPass, ui_example_system)
        // Debugging //
        .add_plugins(WorldInspectorPlugin::new())
        // View Modes //
        .insert_state::<ViewMode>(ViewMode::Graphs2D)
        .add_systems(
            OnEnter(ViewMode::Graphs2D),
            (
                graph2d::setup::setup_system,
                graph2d::axis::setup_axis_system,
            ),
        )
        .add_systems(OnExit(ViewMode::Graphs2D), graph2d::cleanup::cleanup_system)
        .add_systems(
            Update,
            graph2d::axis::update_axis_ticks_system.run_if(in_state(ViewMode::Graphs2D)),
        )
        .run();
}

fn ui_example_system(mut contexts: EguiContexts) -> Result {
    egui::Window::new("Hello").show(contexts.ctx_mut()?, |ui| {
        ui.label("world");
    });
    Ok(())
}
