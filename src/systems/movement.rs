use crate::models::player::{Player, PlayerState};
use crate::systems::collision::is_colliding_with_walls;
use bevy::prelude::*;
use log::debug;

pub fn init(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_positions: Query<&mut Transform, With<Player>>,
    mut player: ResMut<PlayerState>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();

    for mut transform in player_positions.iter_mut() {
        if is_colliding_with_walls(window, &mut transform) {
            return;
        }

        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::S) {
            // && !player.grounded
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Space) {
            transform.translation.y += 2.;
            player.jumping = true;
        } else {
            player.jumping = false;
        }

        debug!(
            "X: {}, Y: {}",
            transform.translation.x, transform.translation.y
        );
    }
}
