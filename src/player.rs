use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_tilemap::prelude::*;

use crate::{MAP_SIZE, MAP_TYPE, TILE_SIZE};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct LastDirection(Vec2);

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server
                .load("sprites/kenney-rtsscifi/PNG/Default size/Unit/scifiUnit_01.png"),
            sprite: Sprite {
                // color: Color::hex("F40404").unwrap(),
                custom_size: Some(Vec2::new(92.0, 92.0)),
                anchor: Anchor::Custom(Vec2::new(0., -0.1)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 2.),
            ..default()
        },
        Player,
        LastDirection(Vec2::splat(0.)),
    ));
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut LastDirection), With<Player>>,
) {
    let (mut transform, mut last_direction) = player_query.single_mut();
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

    transform.translation += movement;
    last_direction.0 = movement.normalize().truncate();

    println!("{:?}", last_direction.0);
}

#[derive(Component)]
pub struct HoverRectangle;

pub fn setup_hover_rectangle(mut commands: Commands, asset_server: Res<AssetServer>) {
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
