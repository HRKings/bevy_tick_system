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
pub struct SimulationState {
    pub current_day: u64,
    pub current_hour: u64,
    pub current_month: u64,
    pub current_year: u64,
    pub ticks_until_next_hour: u64,
}

impl SimulationState {
    pub const TICKS_PER_HOUR: u64 = 2;
    pub const HOURS_PER_DAY: u64 = 24;

    pub const SUNRISE_HOUR: u64 = 6;
    pub const SUNSET_HOUR: u64 = 18;

    pub const DAYS_PER_MONTH: u64 = 30;
    pub const MONTHS_PER_YEAR: u64 = 12;

    pub fn handle_hours(&mut self) -> bool {
        self.ticks_until_next_hour += 1;

        if self.ticks_until_next_hour >= Self::TICKS_PER_HOUR {
            self.ticks_until_next_hour -= Self::TICKS_PER_HOUR;
            self.current_hour += 1;

            return true;
        }

        false
    }

    pub fn handle_days(&mut self) -> bool {
        if self.current_hour >= Self::HOURS_PER_DAY {
            self.current_hour -= Self::HOURS_PER_DAY;
            self.current_day += 1;

            return true;
        }

        false
    }

    pub fn handle_months(&mut self) -> bool {
        if self.current_day >= Self::DAYS_PER_MONTH {
            self.current_day -= Self::DAYS_PER_MONTH;
            self.current_month += 1;

            return true;
        }

        false
    }

    pub fn handle_years(&mut self) -> bool {
        if self.current_month >= Self::MONTHS_PER_YEAR {
            self.current_month -= Self::MONTHS_PER_YEAR;
            self.current_year += 1;

            return true;
        }

        false
    }

    pub fn get_total_hours(&self) -> u128 {
        let years_to_hours = self.current_year as u128
            * Self::MONTHS_PER_YEAR as u128
            * Self::DAYS_PER_MONTH as u128
            * Self::HOURS_PER_DAY as u128;

        let months_to_hours =
            self.current_month as u128 * Self::DAYS_PER_MONTH as u128 * Self::HOURS_PER_DAY as u128;

        let days_to_hours = self.current_day as u128 * Self::HOURS_PER_DAY as u128;

        years_to_hours + months_to_hours + days_to_hours + self.current_hour as u128
    }
}

pub fn tick_simulation(mut state: ResMut<SimulationState>) {
    if state.handle_hours() {
        if state.current_hour == SimulationState::SUNRISE_HOUR {
            //todo
        }

        if state.current_hour == SimulationState::SUNSET_HOUR {
            //todo
        }
    }

    if state.handle_days() {
        //todo
    }

    if state.handle_months() {
        //todo
    }

    if state.handle_years() {
        //todo
    }
}

#[derive(Debug, Default)]
pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationState>()
            .register_type::<SimulationState>()
            .add_plugins(
                ResourceInspectorPlugin::<SimulationState>::default()
                    .run_if(input_toggle_active(true, KeyCode::Escape)),
            )
            .add_systems(FixedUpdate, tick_simulation);
    }
}
