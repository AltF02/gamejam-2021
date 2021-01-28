use bevy::prelude::*;

pub struct Player;
pub struct PlayerMaterial(Handle<ColorMaterial>);
pub struct PlayerState {
    pub jumping: bool,
    pub grounded: bool,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState {
            jumping: false,
            grounded: true,
        }
    }
}

pub fn init(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("sprites/player.png");

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
            sprite: Sprite::new(Vec2::new(90.0, 165.0)),
            ..Default::default()
        })
        .with(Player);
}
