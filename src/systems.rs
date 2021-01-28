pub mod collision;
pub mod gravity;
pub mod movement;

use crate::systems::gravity::GravityTimer;
use bevy::prelude::*;

pub fn init(commands: &mut Commands) {
    commands.insert_resource(GravityTimer(Timer::from_seconds(0.01, true)));
}
