use bevy::prelude::*;

use bevy_ecs_tilemap::{
    prelude::{
        get_tilemap_center_transform, TilemapId, TilemapSize, TilemapTexture, TilemapTileSize,
        TilemapType,
    },
    tiles::{TileBundle, TileFlip, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle, TilemapPlugin,
};
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent, RngPlugin};

const MAP_TILE_WIDTH: u32 = 17;
const MAP_TILE_HEIGHT: u32 = 11;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: String::from("Shock Showdown"),
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(TilemapPlugin)
        .add_plugin(RngPlugin::new().with_rng_seed(123))
        .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        .add_startup_system(setup_camera)
        .add_startup_system(build_map)
        .add_startup_system(spawn_player)
        .add_system(move_player)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.25,
            ..default()
        },
        ..default()
    });
}

fn build_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut global_rng: ResMut<GlobalRng>,
) {
    let grass_tile_handle: Handle<Image> = asset_server.load("images/tile_grass.png");

    let map_size = TilemapSize {
        x: MAP_TILE_WIDTH,
        y: MAP_TILE_HEIGHT,
    };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    let mut rng = RngComponent::from(&mut global_rng);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    flip: TileFlip {
                        x: rng.bool(),
                        y: rng.bool(),
                        d: rng.bool(),
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

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/circle_16x16.png"),
            sprite: Sprite {
                color: Color::hex("F40404").unwrap(),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 1.),
                ..default()
            },
            ..default()
        },
        Player,
    ));
}

#[derive(Component)]
struct Player;

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = player_transform_query.single_mut();

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        player_transform.translation.y += 1.;
    }

    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        player_transform.translation.y -= 1.;
    }

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        player_transform.translation.x -= 1.;
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        player_transform.translation.x += 1.;
    }
}
