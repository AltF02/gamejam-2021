use bevy::prelude::*;
use crate::models::player;

pub fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    player::init(commands, materials);
}
