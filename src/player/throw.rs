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
        let origin_tile = throw_event.origin_tile;

        let mut target = origin_tile + throw_event.direction;
        let throw_target: Option<TilePos> = loop {
            if !target.within_map_bounds(&MAP_SIZE) {
                break None;
            }

            if tile_storage.get(&target).is_some() {
                target = target + throw_event.direction;

                continue;
            }

            break Some(target);
        };

        if throw_target.is_none() {
            continue;
        }

        commands
            .entity(throw_event.player_entity)
            .remove::<Carried>();
        let tile_entity = commands
            .spawn((
                TileBundle {
                    position: throw_target.unwrap(),
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(5),
                    ..default()
                },
                ObjectType::MovableConduit,
                Collider,
            ))
            .id();

        commands.entity(tilemap_entity).add_child(tile_entity);
        tile_storage.set(&throw_target.unwrap(), tile_entity);
    }
}
