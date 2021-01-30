use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;

pub const PLAYER_HEIGHT: f32 = 135.;
pub const PLAYER_WIDTH: f32 = 54.;

pub struct Player;
pub struct PlayerOnMaterial(pub(crate) Handle<ColorMaterial>);
// pub struct PlayerOffMaterial(pub(crate) Handle<ColorMaterial>);

#[derive(Debug)]
pub struct PlayerState {
    pub dead: bool,
    pub jumping: bool,
    pub grounded: bool,
    pub collision: Option<Collision>,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState {
            dead: false,
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
    let texture_on = asset_server.load("sprites/player_on.png");

    commands
        .insert_resource(PlayerOnMaterial(materials.add(texture_on.into())))
        .insert_resource(PlayerState {
            ..Default::default()
        });
}

pub fn spawn(commands: &mut Commands, materials: Res<PlayerOnMaterial>) {
    commands
        .spawn(SpriteBundle {
            material: materials.0.clone(),
            sprite: Sprite::new(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
            transform: Transform::from_translation(Vec3::new(-300., 0., 0.)),
            ..Default::default()
        })
        .with(Player);
}
