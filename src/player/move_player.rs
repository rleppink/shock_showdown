use crate::prelude::*;

use super::{LastDirection, Player};

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
