use crate::prelude::*;

use super::{LastDirection, Player};

pub fn spawn(
    mut commands: Commands,
    player_spawn_query: Query<(&PlayerSpawn, &TilePos)>,
    asset_server: Res<AssetServer>,
) {
    let spawn_p1 = player_spawn_query
        .iter()
        .filter(|(player_spawn, _)| player_spawn.player_number == 1)
        .map(|(_, spawn_tile_pos)| spawn_tile_pos)
        .next()
        .unwrap();

    let spawn_pos_p1 = spawn_p1.center_in_world(&TILE_SIZE.into(), &MAP_TYPE);
    commands.spawn((
        SpriteBundle {
            texture: asset_server
                .load("sprites/kenney-rtsscifi/PNG/Default size/Unit/scifiUnit_01.png"),
            sprite: Sprite {
                color: Color::hex("7777FF").unwrap(),
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..default()
            },
            transform: Transform::from_xyz(spawn_pos_p1.x, spawn_pos_p1.y, 2.),
            ..default()
        },
        Player(1),
        LastDirection(Vec2::splat(0.)),
    ));

    let spawn_p6 = player_spawn_query
        .iter()
        .filter(|(player_spawn, _)| player_spawn.player_number == 6)
        .map(|(_, spawn_tile_pos)| spawn_tile_pos)
        .next()
        .unwrap();

    let spawn_pos_p6 = spawn_p6.center_in_world(&TILE_SIZE.into(), &MAP_TYPE);
    commands.spawn((
        SpriteBundle {
            texture: asset_server
                .load("sprites/kenney-rtsscifi/PNG/Default size/Unit/scifiUnit_01.png"),
            sprite: Sprite {
                color: Color::hex("FF7777").unwrap(),
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..default()
            },
            transform: Transform::from_xyz(spawn_pos_p6.x, spawn_pos_p6.y, 2.),
            ..default()
        },
        Player(6),
        LastDirection(Vec2::splat(0.)),
    ));
}
