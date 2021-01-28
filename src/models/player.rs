use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;

pub const PLAYER_HEIGHT: f32 = 135.;
pub const PLAYER_WIDTH: f32 = 54.;

pub struct Player;
pub struct PlayerMaterial(Handle<ColorMaterial>);
#[derive(Debug)]
pub struct PlayerState {
    pub jumping: bool,
    pub grounded: bool,
    pub collision: Option<Collision>,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState {
            jumping: false,
            grounded: false,
            collision: None,
        }
    }
}

pub fn init(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: AssetServer,
) {
    let texture = asset_server.load("sprites/player_on.png");

    commands
        .insert_resource(PlayerMaterial(materials.add(texture.into())))
        .insert_resource(PlayerState {
            ..Default::default()
        });
}

pub fn spawn(commands: &mut Commands, materials: Res<PlayerMaterial>) {
    commands
        .spawn(SpriteBundle {
            material: materials.0.clone(),
            sprite: Sprite::new(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
            ..Default::default()
        })
        .with(Player);
}

pub fn toggle_jetpack(toggle: bool, sprite: Sprite) {
    let file = if toggle { "toggle_on" } else { "toggle_off" };
}
