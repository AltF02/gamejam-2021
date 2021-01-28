use bevy::prelude::*;

pub struct Item;
pub struct ItemMaterial(Handle<ColorMaterial>);

pub fn init(
    commands: &mut Commands,
    asset_server: AssetServer,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let item_sprite = asset_server.load("sprites/platform.png");
    commands.insert_resource(ItemMaterial(materials.add(item_sprite.into())));
}
