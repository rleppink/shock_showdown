use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, Anchor},
};
use bevy_ecs_tilemap::prelude::*;
use bevy_turborand::prelude::*;

use crate::{collision::Collider, map_builder::PlayerSpawn, MAP_SIZE, MAP_TYPE, TILE_SIZE};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct LastDirection(pub Vec2);

pub fn spawn(
    mut commands: Commands,
    player_spawn_query: Query<(&PlayerSpawn, &TilePos)>,
    asset_server: Res<AssetServer>,
    mut global_rng: ResMut<GlobalRng>,
) {
    let all_spawns: Vec<(&PlayerSpawn, &TilePos)> = player_spawn_query.iter().collect();

    let mut rng = RngComponent::from(&mut global_rng);
    let (_player_spawn, tile_pos) = rng.sample(all_spawns.as_slice()).unwrap();

    let spawn_pos = tile_pos.center_in_world(&TILE_SIZE.into(), &MAP_TYPE);

    commands.spawn((
        SpriteBundle {
            texture: asset_server
                .load("sprites/kenney-rtsscifi/PNG/Default size/Unit/scifiUnit_01.png"),
            sprite: Sprite {
                // color: Color::hex("F40404").unwrap(),
                custom_size: Some(Vec2::new(128.0, 128.0)),
                anchor: Anchor::Custom(Vec2::new(-0.075, -0.1)),
                ..default()
            },
            transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 2.),
            ..default()
        },
        Player,
        LastDirection(Vec2::splat(0.)),
    ));
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut LastDirection), With<Player>>,
    collider_tile_pos_query: Query<&TilePos, (With<Collider>, Without<Player>)>,
) {
    let (mut player_transform, mut last_direction) = player_query.single_mut();
    let mut movement = Vec3::splat(0.);

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        movement = Vec3::new(-4., 0., 0.);
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        movement = Vec3::new(4., 0., 0.);
    }

    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        movement = Vec3::new(0., -4., 0.);
    }

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        movement = Vec3::new(0., 4., 0.);
    }

    let mut collided = false;
    for tile_pos in collider_tile_pos_query.iter() {
        let tile_in_world: Vec2 = tile_pos.center_in_world(&TILE_SIZE.into(), &MAP_TYPE);

        if let Some(_) = collide(
            player_transform.translation + movement,
            Vec2::new(32., 32.),
            tile_in_world.extend(0.),
            Vec2::new(64., 64.),
        ) {
            collided = true;
            break;
        }
    }

    if !collided {
        player_transform.translation += movement;
    }

    if movement.truncate() == Vec2::splat(0.) {
        return;
    }

    last_direction.0 = movement.normalize().truncate();
}

#[derive(Component)]
pub struct HoverRectangle;

pub fn spawn_hover_rectangle(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/outline_yellow_64x64.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
        HoverRectangle,
    ));
}

pub fn draw_hover_rectangle(
    player_query: Query<&Transform, (With<Player>, Without<HoverRectangle>)>,
    mut hover_rectangle_query: Query<&mut Transform, (With<HoverRectangle>, Without<Player>)>,
) {
    let player_transform: Transform = *player_query.single();
    let player_translation_2d = player_transform.translation.truncate();
    let new_tile_pos = TilePos::from_world_pos(
        &player_translation_2d,
        &MAP_SIZE,
        &TILE_SIZE.into(),
        &MAP_TYPE,
    );

    if new_tile_pos.is_none() {
        return;
    }

    let new_tile_world_pos = new_tile_pos
        .unwrap()
        .center_in_world(&TILE_SIZE.into(), &MAP_TYPE);

    let mut hover_rectangle_transform = hover_rectangle_query.single_mut();
    hover_rectangle_transform.translation.x = new_tile_world_pos.x;
    hover_rectangle_transform.translation.y = new_tile_world_pos.y;
}
