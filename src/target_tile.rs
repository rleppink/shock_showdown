use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::{
    player::{LastDirection, Player},
    MAP_SIZE, MAP_TYPE, TILE_SIZE,
};

#[derive(Component, Debug)]
pub struct TargetTile(pub TilePos);

#[allow(clippy::type_complexity)] // It's not that bad, definitely in light of Bevy queries
pub fn update_player_target(
    mut commands: Commands,
    player_query: Query<
        (Entity, &Transform, &LastDirection),
        (With<Player>, Without<TargetTileOutline>),
    >,
) {
    for (player_entity, player_transform, player_last_direction) in player_query.iter() {
        let player_translation = player_transform.translation.truncate();
        let current_tile_pos: TilePos = match TilePos::from_world_pos(
            &player_translation,
            &MAP_SIZE,
            &TILE_SIZE.into(),
            &MAP_TYPE,
        ) {
            Some(tile_pos) => tile_pos,
            None => continue,
        };

        let target_tile_pos = TilePos {
            x: (current_tile_pos.x as f32 + player_last_direction.0.x) as u32,
            y: (current_tile_pos.y as f32 + player_last_direction.0.y) as u32,
        };

        commands
            .entity(player_entity)
            .insert(TargetTile(target_tile_pos));
    }
}

#[derive(Component)]
pub struct TargetTileOutline;

pub fn spawn_target_tile_outline(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/outline_blue_64x64.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
        TargetTileOutline,
    ));
}

pub fn move_target_tile_outline(
    target_tile_query: Query<&TargetTile, (With<Player>, Without<TargetTileOutline>)>,
    mut target_tile_outline_query: Query<
        &mut Transform,
        (With<TargetTileOutline>, Without<Player>),
    >,
) {
    let target_tile = match target_tile_query.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let target_tile_vec2 = target_tile.0.center_in_world(&TILE_SIZE.into(), &MAP_TYPE);

    let mut target_tile = target_tile_outline_query.single_mut();
    target_tile.translation.x = target_tile_vec2.x;
    target_tile.translation.y = target_tile_vec2.y;
}
