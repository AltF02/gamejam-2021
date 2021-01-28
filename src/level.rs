use crate::models::platform;
use crate::models::platform::{PlatformMaterial, PLATFORM_WIDTH};
use bevy::prelude::*;

const LEVEL_FLOOR_WIDTH: i32 = 100;

pub fn init(
    commands: &mut Commands,
    asset_server: AssetServer,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    platform::init(commands, asset_server, materials);
}

pub fn spawn(commands: &mut Commands, materials: Res<PlatformMaterial>) {
    let starting_point = -((LEVEL_FLOOR_WIDTH * PLATFORM_WIDTH as i32) / 2);
    for i in 0..LEVEL_FLOOR_WIDTH {
        let pos = Vec2::new((starting_point + (PLATFORM_WIDTH as i32 * i)) as f32, -300.);
        platform::spawn(commands, &materials, pos)
    }
    // platform::spawn(commands, &materials, Default::default());
}
