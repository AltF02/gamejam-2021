use bevy::prelude::*;

pub struct Plate;
pub struct PlateMaterial(Handle<ColorMaterial>);

pub const PLATE_WIDTH: f32 = 80.;
pub const PLATE_HEIGHT: f32 = 10.;

pub fn init(
    commands: &mut Commands,
    asset_server: &AssetServer,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let plate_sprite = asset_server.load("sprites/plate.png");
    commands.insert_resource(PlateMaterial(materials.add(plate_sprite.into())));
}

pub fn spawn(commands: &mut Commands, materials: &Res<PlateMaterial>, pos: &(f32, f32)) {
    commands
        .insert_resource(Plate)
        .spawn(SpriteBundle {
            material: materials.0.clone(),
            sprite: Sprite::new(Vec2::new(PLATE_WIDTH, PLATE_HEIGHT)),
            transform: Transform::from_translation(Vec3::new(pos.0, pos.1, 0.)),
            ..Default::default()
        })
        .with(Plate);
}
