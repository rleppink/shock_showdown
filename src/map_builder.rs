use std::fs;

use bevy::prelude::*;
use bevy_ecs_tilemap::{helpers, prelude::*};

use crate::{collision::Collider, MAP_SIZE, MAP_TYPE, TILE_SIZE};

#[derive(Component)]
pub struct PlayerSpawn {
    pub player_number: u8,
}

pub fn build_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_image_handle: Handle<Image> = asset_server.load("sprites/tiles.png");

    let basemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(MAP_SIZE);

    helpers::filling::fill_tilemap(
        TileTextureIndex(0),
        MAP_SIZE,
        TilemapId(basemap_entity),
        &mut commands,
        &mut tile_storage,
    );

    commands.entity(basemap_entity).insert(TilemapBundle {
        grid_size: TILE_SIZE.into(),
        map_type: MAP_TYPE,
        size: MAP_SIZE,
        storage: tile_storage,
        texture: TilemapTexture::Single(tile_image_handle.clone()),
        tile_size: TILE_SIZE,
        transform: Transform {
            translation: Vec3 {
                z: 0.0,
                ..default()
            },
            ..default()
        },
        ..default()
    });

    let objects = read_level();
    let objects_map_entity = commands.spawn_empty().id();
    let mut object_storage = TileStorage::empty(MAP_SIZE);
    for ((x, y), object_type) in objects {
        let tile_texture_index = match object_type {
            ObjectType::Empty => continue,
            ObjectType::Wall => 1,
            ObjectType::PowerSource => 2,
            ObjectType::PowerDrain(_) => 3,
            ObjectType::ImmovableConduit => 4,
            ObjectType::MovableConduit => 5,
            ObjectType::Antenna(_) => 6,
            ObjectType::LogicalAnd => 7,
            ObjectType::PlayerSpawn(player_spawn) => {
                // This is most certainly not the right place for this code, but
                // it gets it there. That's fine for now.
                let tile_pos = TilePos {
                    x: x as u32,
                    y: y as u32,
                };

                commands.spawn((
                    TilePos { ..tile_pos },
                    PlayerSpawn {
                        player_number: player_spawn,
                    },
                ));
                continue;
            }
        };

        let tile_pos = TilePos {
            x: x as u32,
            y: y as u32,
        };
        let tile_entity = commands
            .spawn(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(objects_map_entity),
                texture_index: TileTextureIndex(tile_texture_index),
                ..Default::default()
            })
            .id();

        match object_type {
            ObjectType::PlayerSpawn(_) => (),
            ObjectType::Empty => (),
            _ => {
                commands.entity(tile_entity).insert(Collider);
            }
        };

        commands.entity(tile_entity).insert(object_type);
        commands.entity(objects_map_entity).add_child(tile_entity);
        object_storage.set(&tile_pos, tile_entity);
    }

    commands.entity(objects_map_entity).insert(TilemapBundle {
        grid_size: TILE_SIZE.into(),
        map_type: MAP_TYPE,
        size: MAP_SIZE,
        storage: object_storage,
        texture: TilemapTexture::Single(tile_image_handle),
        tile_size: TILE_SIZE,
        transform: Transform {
            translation: Vec3 {
                z: 1.0,
                ..default()
            },
            ..default()
        },
        ..default()
    });
}

#[derive(Component, Debug, PartialEq)]
pub enum ObjectType {
    Empty,
    PowerSource,

    // Some(1) == team 1 power drain
    PowerDrain(Option<u8>),

    // 0109.png -> 109
    Wall,
    MovableConduit,
    ImmovableConduit,
    Antenna(u8),
    PlayerSpawn(u8),
    LogicalAnd,
}

fn read_level() -> Vec<((usize, usize), ObjectType)> {
    const DEFAULT_LEVEL_PATH: &str = "assets/levels/default.txt";
    let default_level = fs::read_to_string(DEFAULT_LEVEL_PATH)
        .unwrap_or_else(|_| panic!("Couldn't read level path: {DEFAULT_LEVEL_PATH}"));

    let mut tiles = Vec::new();
    for (y, line) in default_level.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            let pos = (x, y);
            match character {
                '.' => tiles.push((pos, ObjectType::Empty)),
                's' => tiles.push((pos, ObjectType::PowerSource)),
                'w' => tiles.push((pos, ObjectType::Wall)),
                'C' => tiles.push((pos, ObjectType::MovableConduit)),
                'c' => tiles.push((pos, ObjectType::ImmovableConduit)),
                '!' => tiles.push((pos, ObjectType::Antenna(1))),
                '@' => tiles.push((pos, ObjectType::Antenna(2))),
                '#' => tiles.push((pos, ObjectType::Antenna(3))),
                '$' => tiles.push((pos, ObjectType::Antenna(4))),
                '%' => tiles.push((pos, ObjectType::Antenna(5))),
                '^' => tiles.push((pos, ObjectType::Antenna(6))),
                '1' => tiles.push((pos, ObjectType::PlayerSpawn(1))),
                '2' => tiles.push((pos, ObjectType::PlayerSpawn(2))),
                '3' => tiles.push((pos, ObjectType::PlayerSpawn(3))),
                '4' => tiles.push((pos, ObjectType::PlayerSpawn(4))),
                '5' => tiles.push((pos, ObjectType::PlayerSpawn(5))),
                '6' => tiles.push((pos, ObjectType::PlayerSpawn(6))),
                'a' => tiles.push((pos, ObjectType::LogicalAnd)),
                'd' => tiles.push((pos, ObjectType::PowerDrain(None))),
                '~' => tiles.push((pos, ObjectType::PowerDrain(Some(1)))),
                ')' => tiles.push((pos, ObjectType::PowerDrain(Some(2)))),
                c => panic!("Unknown character: '{c}' at {pos:?}"),
            }
        }
    }

    tiles
}
