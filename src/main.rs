mod simulation;
mod stats_debugger;
mod tick_engine;

use std::{
    thread,
    time::{Duration, Instant},
};

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

use crate::{simulation::SimulationPlugin, tick_engine::TickEnginePlugin};
use crate::stats_debugger::StatsDebuggerPlugin;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn(Camera2dBundle::default());
    // Text with one section
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "hello\nbevy!",
            TextStyle {
                font_size: 100.0,
                ..default()
            },
        )
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        ColorText,
    ));
}

fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
    for mut text in &mut query {
        let seconds = time.elapsed_seconds();

        // Update the color of the first and only section.
        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
        .add_plugins(StatsDebuggerPlugin)
        .add_plugins(SimulationPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, text_color_system);

    TickEnginePlugin::default().build(&mut app);

    app.run();
}
