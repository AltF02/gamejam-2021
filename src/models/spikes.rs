use bevy::prelude::*;

pub struct Spikes;
pub struct SpikesMaterial(Handle<ColorMaterial>);

pub const SPIKES_WIDTH: f32 = 300.;
pub const SPIKES_HEIGHT: f32 = 33.;

pub fn init(
    commands: &mut Commands,
    asset_server: &AssetServer,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let spikes_sprite = asset_server.load("sprites/spikes.png");
    commands.insert_resource(SpikesMaterial(materials.add(spikes_sprite.into())));
}

pub fn spawn(
    commands: &mut Commands,
    materials: &Res<SpikesMaterial>,
    pos: &(f32, f32),
    degrees: f32,
) {
    let mut transform = Transform::from_translation(Vec3::new(pos.0, pos.1, 0.));
    transform.rotation = Quat::from_rotation_z(degrees * (std::f32::consts::PI / 180.));
    commands
        .insert_resource(Spikes)
        .spawn(SpriteBundle {
            material: materials.0.clone(),
            sprite: Sprite::new(Vec2::new(SPIKES_WIDTH, SPIKES_HEIGHT)),
            transform,
            ..Default::default()
        })
        .with(Spikes);
}
