use crate::models::player::{Player, PlayerState};
use bevy::prelude::*;

pub struct GravityLevel(pub f32);

pub fn init(
    mut player_positions: Query<&mut Transform, With<Player>>,
    mut player_state: ResMut<PlayerState>,
    mut level: Res<GravityLevel>,
) {
    if player_state.jumping || player_state.grounded {
        return;
    }

    for mut transform in player_positions.iter_mut() {
        transform.translation.y -= level.0;
    }
}
