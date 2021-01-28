use crate::level;
use crate::models::platform::PlatformMaterial;
use crate::models::player;
use bevy::prelude::*;

pub fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    player::init(commands, &mut materials, asset_server.clone());
    level::init(commands, asset_server.clone(), &mut materials);
}
