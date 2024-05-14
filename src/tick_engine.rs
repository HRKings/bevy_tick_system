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

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct TickEnginePlugin {
    #[inspector(min = 1, max = 1_000_000)]
    target_tps: i32,
    current_tps: f64,
}

impl Default for TickEnginePlugin {
    fn default() -> Self {
        Self {
            target_tps: 100,
            current_tps: Default::default(),
        }
    }
}

impl TickEnginePlugin {
    pub fn get_time_delta(&self) -> f64 {
        1.0 / self.target_tps as f64
    }
}

#[derive(Resource)]
struct TickHistory {
    timer: Instant,
    counter: usize,
    tick_delta: usize,
    seconds_elapsed: f64,
}

fn check_target_tps_changed(
    tick_engine_state: Res<TickEnginePlugin>,
    mut fixed_time: ResMut<Time<Fixed>>,
) {
    if tick_engine_state.is_changed() {
        fixed_time.set_timestep_seconds(tick_engine_state.get_time_delta())
    }
}

fn tps_preupdate_system(mut history: ResMut<TickHistory>) {
    history.tick_delta = 0;
    history.seconds_elapsed = 0.0;
}

fn tps_update_system(
    fixed_time: Res<Time<Fixed>>,
    mut history: ResMut<TickHistory>,
    mut tick_engine_state: ResMut<TickEnginePlugin>,
) {
    let tps = 1.0 / fixed_time.delta_seconds_f64();

    history.counter += 1;
    history.tick_delta += 1;

    tick_engine_state.current_tps = tps;
}

impl Plugin for TickEnginePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TickEnginePlugin>()
            .register_type::<TickEnginePlugin>()
            .add_plugins(
                ResourceInspectorPlugin::<TickEnginePlugin>::default()
                    .run_if(input_toggle_active(true, KeyCode::Escape)),
            )
            .add_systems(PreUpdate, tps_preupdate_system)
            .add_systems(FixedUpdate, tps_update_system)
            .insert_resource(Time::<Fixed>::from_seconds(self.get_time_delta()))
            .insert_resource(TickHistory {
                timer: Instant::now(),
                counter: 0,
                tick_delta: 0,
                seconds_elapsed: 0.0,
            })
            .add_systems(Update, check_target_tps_changed);
    }
}
