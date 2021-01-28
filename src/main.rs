mod models;
mod setup;
mod systems;

use crate::models::player;
use crate::setup::setup;
use crate::systems::{gravity, movement};

use bevy::prelude::*;
use bevy_diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};
use log::LevelFilter;
use simple_logger::SimpleLogger;

struct SnakeHead;
struct Materials {
    head_material: Handle<ColorMaterial>,
}

#[bevy_main]
fn main() {
    #[cfg(debug_assertions)]
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    #[cfg(not(debug_assertions))]
    SimpleLogger::new()
        .with_level(LevelFilter::Error)
        .init()
        .unwrap();

    App::build()
        .add_resource(WindowDescriptor {
            title: "GameJam 2021 :D".to_string(),
            vsync: true,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(player::spawn.system()))
        .add_system(movement::init.system())
        .add_system(gravity::init.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .run();
}
