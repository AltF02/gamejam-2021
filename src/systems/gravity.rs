use crate::models::player::{Player, PlayerState};
use bevy::prelude::*;

pub struct GravityLevel(pub f32);

pub fn init(
    mut player_positions: Query<&mut Transform, With<Player>>,
    player_state: ResMut<PlayerState>,
    level: Res<GravityLevel>,
) {
    if player_state.jumping || player_state.grounded || player_state.dead {
        return;
    }

    for mut transform in player_positions.iter_mut() {
        transform.translation.y -= level.0;
    }
}
