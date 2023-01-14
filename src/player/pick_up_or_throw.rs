use crate::prelude::*;

use super::{Carried, Player, ThrowEvent};

// This function really needs some refactoring, it's doing too much
pub fn pick_up_or_throw(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    target_tile_query: Query<&TargetTile>,
    tile_query: Query<(Entity, &TilePos, &ObjectType), With<TilemapId>>,
    carried_query: Query<(Entity, Option<&Carried>), With<Player>>,
    mut tile_storage_query: Query<(&mut TileStorage, &Transform)>,
    mut throw_event_writer: EventWriter<ThrowEvent>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    let (player_entity, maybe_carried) = carried_query.single();
    let target_tile_pos = target_tile_query.single().0;
    match maybe_carried {
        Some(_) => {
            throw_event_writer.send(ThrowEvent(player_entity, target_tile_pos));
        }
        None => {
            for (tile_entity, tile_position, object_type) in tile_query.iter() {
                if target_tile_pos != *tile_position {
                    continue;
                }

                if let ObjectType::MovableConduit = object_type {
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
