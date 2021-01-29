use crate::models::explosion;
use crate::models::explosion::ExplosionMaterial;
use crate::models::player::{Player, PlayerState};
use crate::models::spikes::Spikes;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub fn init(
    mut player_positions: Query<(&mut Transform, &Sprite), With<Player>>,
    commands: &mut Commands,
    mut spikes: Query<(&mut Transform, &Sprite), With<Spikes>>,
    mut player: ResMut<PlayerState>,
    materials: Res<ExplosionMaterial>,
) {
    for (player_transform, player_sprite) in player_positions.iter_mut() {
        for (spikes_transform, spikes_sprite) in spikes.iter_mut() {
            let collision = collide(
                player_transform.translation,
                player_sprite.size,
                spikes_transform.translation,
                spikes_sprite.size,
            );

            if collision.is_some() && !player.dead {
                player.dead = true;
                explosion::spawn(commands, &materials);
            }
        }
    }
}
