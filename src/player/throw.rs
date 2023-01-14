use crate::prelude::*;

use super::{Carried, ThrowEvent};

pub fn throw_blocks(
    mut throw_events: EventReader<ThrowEvent>,
    mut tilemap_query: Query<(Entity, &mut TileStorage, &Transform)>,
    mut commands: Commands,
) {
    let (tilemap_entity, mut tile_storage, _) = tilemap_query
        .iter_mut()
        .find(|(_, _, transform)| transform.translation.z == 1.)
        .expect("There should only be one layer 1 tile storage");

    for throw_event in throw_events.iter() {
        let player_entity = throw_event.0;
        let throw_target = throw_event.1;

        if tile_storage.get(&throw_target).is_some() {
            // We don't want to throw on top of other blocks
            continue;
        }

        commands.entity(player_entity).remove::<Carried>();
        let tile_entity = commands
            .spawn((
                TileBundle {
                    position: throw_target,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(5),
                    ..default()
                },
                ObjectType::MovableConduit,
                Collider,
            ))
            .id();

        commands.entity(tilemap_entity).add_child(tile_entity);
        tile_storage.set(&throw_target, tile_entity);
    }
}
