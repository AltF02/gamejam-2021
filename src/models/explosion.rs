use bevy::prelude::*;

pub struct Explosion;
pub struct ExplosionMaterial(Handle<ColorMaterial>);

pub const EXPLOSION_WIDTH: f32 = 200.;
pub const EXPLOSION_HEIGHT: f32 = 200.;

pub fn init(
    commands: &mut Commands,
    asset_server: &AssetServer,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let plate_sprite = asset_server.load("sprites/explosion.png");
    commands.insert_resource(ExplosionMaterial(materials.add(plate_sprite.into())));
}

pub fn spawn(commands: &mut Commands, materials: &Res<ExplosionMaterial>) {
    commands
        .insert_resource(Explosion)
        .spawn(SpriteBundle {
            material: materials.0.clone(),
            sprite: Sprite::new(Vec2::new(EXPLOSION_WIDTH, EXPLOSION_HEIGHT)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..Default::default()
        })
        .with(Explosion);
}
