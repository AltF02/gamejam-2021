use crate::models::platform::{PlatformMaterial, PLATFORM_WIDTH};
use crate::models::pressure_plate;
use crate::models::pressure_plate::{Plate, PlateMaterial};
use crate::models::{explosion, platform};
use crate::systems::gravity::GravityLevel;
use bevy::prelude::*;

use crate::models::explosion::Explosion;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

const LEVEL_FLOOR_WIDTH: i32 = 10;
const LEVEL_PLATFORM_LOCATIONS: [(f32, f32); 3] = [(-450., 220.), (450., -260.), (125., 140.)];

pub fn init(
    commands: &mut Commands,
    asset_server: AssetServer,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(GravityLevel(thread_rng().gen_range(1.0..6.0)));
    platform::init(commands, &asset_server, materials);
    pressure_plate::init(commands, &asset_server, materials);
    explosion::init(commands, &asset_server, materials);
}

pub fn spawn(
    commands: &mut Commands,
    platform_materials: Res<PlatformMaterial>,
    plate_materials: Res<PlateMaterial>,
    mut explosion: Query<Entity, With<Explosion>>,
    mut plate: Query<Entity, With<Plate>>,
) {
    let starting_point = -((LEVEL_FLOOR_WIDTH * PLATFORM_WIDTH as i32) / 2);
    for i in 0..LEVEL_FLOOR_WIDTH {
        let pos = Vec2::new((starting_point + (PLATFORM_WIDTH as i32 * i)) as f32, -300.);
        platform::spawn(commands, &platform_materials, pos)
    }

    platform::spawn(commands, &platform_materials, Vec2::new(100., 100.));
    platform::spawn(commands, &platform_materials, Vec2::new(-420., 180.));
    platform::spawn(commands, &platform_materials, Vec2::new(-820., 180.));
    reset(commands, &mut plate, &mut explosion, &plate_materials);
    // pressure_plate::spawn(commands, &plate_materials, &(125., 140.));
}

pub fn reset(
    commands: &mut Commands,
    mut plate: &mut Query<Entity, With<Plate>>,
    mut explosion: &mut Query<Entity, With<Explosion>>,
    plate_materials: &Res<PlateMaterial>,
) {
    for entity in plate.iter_mut() {
        commands.remove::<SpriteBundle>(entity);
    }

    for entity in explosion.iter_mut() {
        commands.remove::<SpriteBundle>(entity);
    }

    pressure_plate::spawn(
        commands,
        &plate_materials,
        LEVEL_PLATFORM_LOCATIONS.choose(&mut thread_rng()).unwrap(),
    );
}
