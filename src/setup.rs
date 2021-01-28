use crate::models::player;
use bevy::prelude::*;

pub fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    player::init(commands, materials, asset_server);
}
