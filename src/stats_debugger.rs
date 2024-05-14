use std::time::Instant;
use bevy::{
    app::FixedMain,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    ecs::component::Tick,
    prelude::*,
};

use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::inspector_egui_impls::InspectorPrimitive;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct StatsDebugger {
    current_fps: f64,
}

fn fps_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut stats_debugger: ResMut<StatsDebugger>,
) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            stats_debugger.current_fps = value;
        }
    }
}

#[derive(Debug, Default)]
pub struct StatsDebuggerPlugin;

impl Plugin for StatsDebuggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            ResourceInspectorPlugin::<StatsDebugger>::default()
                .run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .init_resource::<StatsDebugger>()
        .register_type::<StatsDebugger>()
        .add_systems(Update, fps_update_system);
    }
}
