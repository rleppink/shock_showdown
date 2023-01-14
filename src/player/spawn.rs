use crate::prelude::*;

use super::{LastDirection, Player};

pub fn spawn(
    mut commands: Commands,
    player_spawn_query: Query<(&PlayerSpawn, &TilePos)>,
    asset_server: Res<AssetServer>,
    mut global_rng: ResMut<GlobalRng>,
) {
    let all_spawns: Vec<(&PlayerSpawn, &TilePos)> = player_spawn_query.iter().collect();

    let mut rng = RngComponent::from(&mut global_rng);
    let (_player_spawn, tile_pos) = rng.sample(all_spawns.as_slice()).unwrap();

    let spawn_pos = tile_pos.center_in_world(&TILE_SIZE.into(), &MAP_TYPE);

    commands.spawn((
        SpriteBundle {
            texture: asset_server
                .load("sprites/kenney-rtsscifi/PNG/Default size/Unit/scifiUnit_01.png"),
            sprite: Sprite {
                // color: Color::hex("F40404").unwrap(),
                custom_size: Some(Vec2::new(128.0, 128.0)),
                // anchor: Anchor::Custom(Vec2::new(-0.075, -0.1)),
                ..default()
            },
            transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 2.),
            ..default()
        },
        Player,
        LastDirection(Vec2::splat(0.)),
    ));
}
