use crate::models::platform::{PlatformMaterial, PLATFORM_WIDTH};
use crate::models::pressure_plate;
use crate::models::pressure_plate::PlateMaterial;
use crate::models::{explosion, platform};
use bevy::prelude::*;

const LEVEL_FLOOR_WIDTH: i32 = 10;

pub fn init(
    commands: &mut Commands,
    asset_server: AssetServer,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    platform::init(commands, &asset_server, materials);
    pressure_plate::init(commands, &asset_server, materials);
    explosion::init(commands, &asset_server, materials);
}

pub fn spawn(
    commands: &mut Commands,
    platform_materials: Res<PlatformMaterial>,
    plate_materials: Res<PlateMaterial>,
) {
    let starting_point = -((LEVEL_FLOOR_WIDTH * PLATFORM_WIDTH as i32) / 2);
    for i in 0..LEVEL_FLOOR_WIDTH {
        let pos = Vec2::new((starting_point + (PLATFORM_WIDTH as i32 * i)) as f32, -300.);
        platform::spawn(commands, &platform_materials, pos)
    }
    platform::spawn(commands, &platform_materials, Vec2::new(100., 120.));
    pressure_plate::spawn(commands, &plate_materials, Vec2::new(-200., -260.));
}

pub fn init_background() {}
