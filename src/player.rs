use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
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

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = player_transform_query.single_mut();
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

    player_transform.translation += movement;
}
