use bevy::prelude::*;
use bevy_ecs_tilemap::{helpers, prelude::*};

use crate::{MAP_SIZE, MAP_TYPE, TILE_SIZE};

pub fn build_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_image_handle: Handle<Image> =
        asset_server.load("sprites/kenney_rpgurbanpack/Tilemap/tilemap_packed_64x64.png");

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(MAP_SIZE);

    helpers::filling::fill_tilemap(
        TileTextureIndex(28),
        MAP_SIZE,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TILE_SIZE.into(),
        map_type: MAP_TYPE,
        size: MAP_SIZE,
        storage: tile_storage,
        texture: TilemapTexture::Single(tile_image_handle),
        tile_size: TILE_SIZE,
        // transform: get_tilemap_center_transform(&MAP_SIZE, &TILE_SIZE.into(), &MAP_TYPE, 0.),
        ..default()
    });
}
