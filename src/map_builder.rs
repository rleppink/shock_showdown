use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_turborand::prelude::*;

pub fn build_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut global_rng: ResMut<GlobalRng>,
) {
    let grass_tile_handle: Handle<Image> = asset_server.load("images/tiles.png");

    let map_size = TilemapSize {
        x: crate::MAP_TILE_WIDTH,
        y: crate::MAP_TILE_HEIGHT,
    };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    let mut rng = RngComponent::from(&mut global_rng);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_texture_index = match (x, y) {
                (8, 0) => TileTextureIndex(2),
                (8, 10) => TileTextureIndex(2),

                (3, 3) => TileTextureIndex(2),
                (3, 5) => TileTextureIndex(2),
                (3, 7) => TileTextureIndex(2),

                (13, 3) => TileTextureIndex(2),
                (13, 5) => TileTextureIndex(2),
                (13, 7) => TileTextureIndex(2),

                (8, _) => TileTextureIndex(2),
                (_, 1) => TileTextureIndex(1),
                (_, 9) => TileTextureIndex(1),
                (3, _) => TileTextureIndex(1),
                (13, _) => TileTextureIndex(1),
                _ => TileTextureIndex(0),
            };

            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: tile_texture_index,
                    flip: TileFlip {
                        x: rng.bool(),
                        y: rng.bool(),
                        ..default()
                    },
                    ..default()
                })
                .id();

            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16., y: 16. };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(grass_tile_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.),
        ..default()
    });
}
