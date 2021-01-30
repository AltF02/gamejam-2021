use crate::models::platform::{PlatformMaterial, PLATFORM_WIDTH};
use crate::models::pressure_plate::{Plate, PlateMaterial};
use crate::models::{explosion, platform};
use crate::models::{pressure_plate, spikes};
use crate::systems::gravity::GravityLevel;
use bevy::prelude::*;

use crate::models::explosion::Explosion;
use crate::models::spikes::SpikesMaterial;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

const LEVEL_FLOOR_WIDTH: i32 = 10;
const LEVEL_PLATFORM_LOCATIONS: [(f32, f32); 4] =
    [(-450., 220.), (450., -260.), (125., 140.), (-450., -260.)];

pub fn init(
    commands: &mut Commands,
    asset_server: AssetServer,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(GravityLevel(thread_rng().gen_range(1.0..6.0)));
    platform::init(commands, &asset_server, materials);
    pressure_plate::init(commands, &asset_server, materials);
    explosion::init(commands, &asset_server, materials);
    spikes::init(commands, &asset_server, materials);
}

pub fn spawn(
    commands: &mut Commands,
    platform_materials: Res<PlatformMaterial>,
    plate_materials: Res<PlateMaterial>,
    spikes_materials: Res<SpikesMaterial>,
    mut explosion: Query<Entity, With<Explosion>>,
    mut plate: Query<Entity, With<Plate>>,
    mut gravity: ResMut<GravityLevel>,
) {
    let starting_point = -((LEVEL_FLOOR_WIDTH * PLATFORM_WIDTH as i32) / 2);
    for i in 0..LEVEL_FLOOR_WIDTH {
        let pos = Vec2::new((starting_point + (PLATFORM_WIDTH as i32 * i)) as f32, -300.);
        platform::spawn(commands, &platform_materials, pos)
    }

    platform::spawn(commands, &platform_materials, Vec2::new(100., 100.));
    platform::spawn(commands, &platform_materials, Vec2::new(-420., 180.));
    platform::spawn(commands, &platform_materials, Vec2::new(-820., 180.));
    spikes::spawn(commands, &spikes_materials, &(-420., 130.), 180.);
    spikes::spawn(commands, &spikes_materials, &(30., -250.), 0.);
    reset(
        commands,
        &mut plate,
        &mut explosion,
        &plate_materials,
        &mut gravity,
    );
    // pressure_plate::spawn(commands, &plate_materials, &(125., 140.));
}

pub fn reset(
    commands: &mut Commands,
    plate: &mut Query<Entity, With<Plate>>,
    explosion: &mut Query<Entity, With<Explosion>>,
    plate_materials: &Res<PlateMaterial>,
    gravity: &mut ResMut<GravityLevel>,
) -> bool {
    let mut point = false;
    if let Some(entity) = plate.iter_mut().next() {
        commands.remove::<SpriteBundle>(entity);
        point = true;
    }

    for entity in explosion.iter_mut() {
        commands.remove_one::<SpriteBundle>(entity);
    }

    pressure_plate::spawn(
        commands,
        &plate_materials,
        LEVEL_PLATFORM_LOCATIONS.choose(&mut thread_rng()).unwrap(),
    );
    gravity.0 = thread_rng().gen_range(1.0..6.0);
    point
}
