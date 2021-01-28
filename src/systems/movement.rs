use crate::models::platform::Platform;
use crate::models::player::{Player, PlayerState};
use crate::systems::collision::{is_colliding_with_platform, is_colliding_with_walls};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use log::debug;

const SPEED: f32 = 3.;

pub fn init(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_positions: Query<(&mut Transform, &Sprite), With<Player>>,
    mut player: ResMut<PlayerState>,
    mut windows: ResMut<Windows>,
    mut platforms: Query<(&mut Transform, &Sprite), With<Platform>>,
) {
    let window = windows.get_primary_mut().unwrap();

    for (mut transform, player_sprite) in player_positions.iter_mut() {
        if is_colliding_with_walls(window, &mut transform) {
            return;
        }

        if !player.grounded {
            for (platform, sprite) in platforms.iter_mut() {
                let collisions = collide(
                    platform.translation,
                    sprite.size,
                    transform.translation,
                    player_sprite.size,
                );

                if collisions.is_some() {
                    player.grounded = true;
                    return;
                }

                player.grounded = false;
            }
        }

        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= SPEED;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += SPEED;
        }
        if keyboard_input.pressed(KeyCode::S) && !player.grounded {
            transform.translation.y -= SPEED;
        }

        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Space) {
            transform.translation.y += SPEED;
            player.jumping = true;
            player.grounded = false;
        } else {
            player.jumping = false;
        }

        // debug!(
        //     "X: {}, Y: {}",
        //     transform.translation.x, transform.translation.y
        // );
    }
}
