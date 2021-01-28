use crate::models::player::Player;
use crate::models::pressure_plate::Plate;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::models::explosion;
use crate::models::explosion::{Explosion, ExplosionMaterial};

use log::debug;

pub fn init(
    mut player_positions: Query<(&mut Transform, &Sprite), With<Player>>,
    mut plates: Query<(&mut Transform, &Sprite), With<Plate>>,
    commands: &mut Commands,
    materials: Res<ExplosionMaterial>,
) {
    for (player_transform, player_sprite) in player_positions.iter_mut() {
        for (plate_transform, plate_sprite) in plates.iter_mut() {
            let collision = collide(
                player_transform.translation,
                player_sprite.size,
                plate_transform.translation,
                plate_sprite.size,
            );

            if collision.is_some() {
                explosion::spawn(commands, &materials);
            }

            debug!("{:?}", collision);
        }
    }
}
