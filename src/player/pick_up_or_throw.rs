use crate::prelude::*;

use super::{Carried, Player, ThrowEvent};

// This function really needs some refactoring, it's doing too much
pub fn pick_up_or_throw(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    tile_query: Query<(Entity, &TilePos, &ObjectType), With<TilemapId>>,
    player_query: Query<
        (
            Entity,
            Option<&Carried>,
            &TilePos,
            &TargetTile,
            &LastDirection,
        ),
        With<Player>,
    >,
    mut tile_storage_query: Query<(&mut TileStorage, &Transform)>,
    mut throw_event_writer: EventWriter<ThrowEvent>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    for (
        player_entity,
        maybe_carried,
        player_tile_pos,
        player_target_tile,
        player_last_direction,
    ) in player_query.iter()
    {
        match maybe_carried {
            Some(_) => {
                throw_event_writer.send(ThrowEvent {
                    player_entity,
                    origin_tile: *player_tile_pos,
                    direction: player_last_direction.0.into(),
                });
            }
            None => {
                for (tile_entity, tile_position, object_type) in tile_query.iter() {
                    if let ObjectType::MovableConduit = object_type {
                        if player_target_tile.0 != *tile_position {
                            continue;
                        }

                        commands.entity(player_entity).insert(Carried(*object_type));

                        commands.entity(tile_entity).despawn_recursive();
                        for (mut tile_storage, transform) in tile_storage_query.iter_mut() {
                            if transform.translation.z == 0. {
                                continue;
                            }

                            tile_storage.remove(tile_position);
                        }
                    } else {
                        continue;
                    }
                }
            }
        }
    }
}
