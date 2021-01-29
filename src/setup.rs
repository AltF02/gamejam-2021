use crate::level;
use crate::models::player;
use crate::models::points::Points;
use crate::ui;
use bevy::prelude::*;

pub fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(Points(0));
    player::init(commands, &mut materials, asset_server.clone());
    level::init(commands, asset_server.clone(), &mut materials);
    ui::init(commands, asset_server.clone());
}
