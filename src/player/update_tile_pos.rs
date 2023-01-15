use crate::prelude::*;

pub fn update_players_tile_pos(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    for (player_entity, player_transform) in player_query.iter() {
        let player_tile_pos = TilePos::from_world_pos(
            &player_transform.translation.truncate(),
            &MAP_SIZE,
            &TILE_SIZE.into(),
            &MAP_TYPE,
        );

        if player_tile_pos.is_none() {
            continue;
        }

        commands
            .entity(player_entity)
            .insert(player_tile_pos.unwrap());
    }
}
