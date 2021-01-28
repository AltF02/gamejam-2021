use bevy::prelude::*;

pub const PLATFORM_WIDTH: f32 = 400.;
pub const PLATFORM_HEIGHT: f32 = 76.;

pub struct Platform;

pub struct PlatformMaterial(Handle<ColorMaterial>);

pub fn init(
    commands: &mut Commands,
    asset_server: AssetServer,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let platform_sprite = asset_server.load("sprites/platform.png");
    commands.insert_resource(PlatformMaterial(materials.add(platform_sprite.into())));
}

pub fn spawn(commands: &mut Commands, materials: &Res<PlatformMaterial>, pos: Vec2) {
    commands
        .insert_resource(Platform)
        .spawn(SpriteBundle {
            material: materials.0.clone(),
            sprite: Sprite::new(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.)),
            ..Default::default()
        })
        .with(Platform);
}
