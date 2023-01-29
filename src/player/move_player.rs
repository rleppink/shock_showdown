use crate::prelude::*;

use super::{LastDirection, Player};

pub fn move_players(
    keyboard_input: Res<Input<KeyCode>>,
    player_keymaps: Res<PlayerKeyMaps>,
    mut player_query: Query<(&Player, &mut Transform, &mut LastDirection)>,
    collider_tile_pos_query: Query<&TilePos, (With<Collider>, Without<Player>)>,
) {
    for player_keymap in &player_keymaps.0 {
        for (key, action) in &player_keymap.key_map {
            if !keyboard_input.pressed(*key) {
                continue;
            }

            let (mut player_transform, mut last_direction) = player_query
                .iter_mut()
                .find_map(|(player, player_transform, last_direction)| {
                    if player.0 == player_keymap.player_number {
                        Some((player_transform, last_direction))
                    } else {
                        None
                    }
                })
                .unwrap();

            let movement = match action {
                Action::MoveUp => Vec3::new(0.0, 1.0, 0.0),
                Action::MoveDown => Vec3::new(0.0, -1.0, 0.0),
                Action::MoveLeft => Vec3::new(-1.0, 0.0, 0.0),
                Action::MoveRight => Vec3::new(1.0, 0.0, 0.0),
                Action::PickUpThrow => Vec3::new(0.0, 0.0, 0.0),
            };

            let mut collided = false;
            for tile_pos in collider_tile_pos_query.iter() {
                let tile_in_world: Vec2 = tile_pos.center_in_world(&TILE_SIZE.into(), &MAP_TYPE);

                if bevy::sprite::collide_aabb::collide(
                    player_transform.translation + movement,
                    Vec2::new(32., 32.),
                    tile_in_world.extend(0.),
                    Vec2::new(64., 64.),
                )
                .is_some()
                {
                    collided = true;
                    break;
                }
            }

            if !collided {
                player_transform.translation += movement * 4.;
            }

            if movement.truncate() == Vec2::splat(0.) {
                continue;
            }

            last_direction.0 = movement.normalize().truncate();
            break;
        }
    }
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform, &mut LastDirection)>,
    collider_tile_pos_query: Query<&TilePos, (With<Collider>, Without<Player>)>,
) {
    let (mut player_transform, mut last_direction) = player_query
        .iter_mut()
        .find_map(|(player, player_transform, last_direction)| {
            if player.0 == 1 {
                Some((player_transform, last_direction))
            } else {
                None
            }
        })
        .unwrap();

    let mut movement = Vec3::splat(0.);

    if keyboard_input.pressed(KeyCode::Left) {
        movement = Vec3::new(-4., 0., 0.);
    }

    if keyboard_input.pressed(KeyCode::Right) {
        movement = Vec3::new(4., 0., 0.);
    }

    if keyboard_input.pressed(KeyCode::Down) {
        movement = Vec3::new(0., -4., 0.);
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement = Vec3::new(0., 4., 0.);
    }

    let mut collided = false;
    for tile_pos in collider_tile_pos_query.iter() {
        let tile_in_world: Vec2 = tile_pos.center_in_world(&TILE_SIZE.into(), &MAP_TYPE);

        if bevy::sprite::collide_aabb::collide(
            player_transform.translation + movement,
            Vec2::new(32., 32.),
            tile_in_world.extend(0.),
            Vec2::new(64., 64.),
        )
        .is_some()
        {
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
