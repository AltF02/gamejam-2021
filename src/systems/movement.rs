use crate::models::platform::Platform;
use crate::models::player::{Player, PlayerState};
use crate::systems::collision::is_colliding_with_walls;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use log::debug;
use std::mem::discriminant;

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

        let mut collided: Option<Collision> = None;
        player.grounded = false;

        for (platform, sprite) in platforms.iter_mut() {
            let collisions = collide(
                platform.translation,
                sprite.size,
                transform.translation,
                player_sprite.size,
            );

            if let Some(collision) = collisions {
                collided = Some(collision);
            }
        }

        if let Some(collision) = &collided {
            match collision {
                Collision::Bottom => player.grounded = true,
                Collision::Top => transform.translation.y -= 0.5,
                Collision::Left => transform.translation.x += 0.5,
                Collision::Right => transform.translation.x -= 0.5,
                _ => {}
            }
        }

        let collision = collided.unwrap_or(Collision::Bottom);

        // debug!("{:?}", &collision);

        if keyboard_input.pressed(KeyCode::A)
            && discriminant(&Collision::Left) != discriminant(&collision)
        {
            transform.translation.x -= SPEED;
        }
        if keyboard_input.pressed(KeyCode::D)
            && discriminant(&Collision::Right) != discriminant(&collision)
        {
            transform.translation.x += SPEED;
        }

        if keyboard_input.pressed(KeyCode::W)
            && discriminant(&Collision::Top) != discriminant(&collision)
        {
            transform.translation.y += SPEED;
            player.jumping = true;
            player.grounded = false;
        } else {
            player.jumping = false;
        }

        debug!(
            "X: {}, Y: {}",
            transform.translation.x, transform.translation.y
        );
    }
}
