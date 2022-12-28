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
                translation: Vec3::new(-64., 0., 1.),
                ..default()
            },
            ..default()
        },
        Player,
    ));
}

pub fn move_(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = player_transform_query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::W) {
        player_transform.translation.y += 16.;
    }

    if keyboard_input.just_pressed(KeyCode::Down) || keyboard_input.just_pressed(KeyCode::S) {
        player_transform.translation.y -= 16.;
    }

    if keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::A) {
        player_transform.translation.x -= 16.;
    }

    if keyboard_input.just_pressed(KeyCode::Right) || keyboard_input.just_pressed(KeyCode::D) {
        player_transform.translation.x += 16.;
    }
}
