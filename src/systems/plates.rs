use crate::models::player::Player;
use crate::models::pressure_plate::{Plate, PlateMaterial};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::models::explosion::Explosion;

use crate::level;
use crate::models::points::Points;
use crate::systems::gravity::GravityLevel;

pub fn init(
    mut player_positions: Query<(&mut Transform, &Sprite), With<Player>>,
    mut plates: Query<(&mut Transform, &Sprite), With<Plate>>,
    commands: &mut Commands,
    mut plate: Query<Entity, With<Plate>>,
    mut explosion: Query<Entity, With<Explosion>>,
    plate_materials: Res<PlateMaterial>,
    mut gravity: ResMut<GravityLevel>,
    mut points: ResMut<Points>,
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
                // FIXME Is sometimes called multiple times
                let point = level::reset(
                    commands,
                    &mut plate,
                    &mut explosion,
                    &plate_materials,
                    &mut gravity,
                );
                if point {
                    points.0 += 1;
                }
            }
        }
    }
}
