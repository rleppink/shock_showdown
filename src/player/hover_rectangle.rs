use crate::prelude::*;

use super::{HoverRectangle, Player};

pub fn spawn_hover_rectangle(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/outline_yellow_64x64.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
        HoverRectangle(1),
    ));
}

pub fn draw_hover_rectangle(
    player_query: Query<(&Player, &Transform), Without<HoverRectangle>>,
    mut hover_rectangle_query: Query<(&mut Transform, &HoverRectangle), Without<Player>>,
) {
    let player_transform: Transform = *player_query
        .iter()
        .find_map(
            |(player, transform)| {
                if player.0 == 1 {
                    Some(transform)
                } else {
                    None
                }
            },
        )
        .unwrap();

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

    let mut hover_rectangle_transform = hover_rectangle_query
        .iter_mut()
        .find_map(|(transform, hover_rectangle)| {
            if hover_rectangle.0 == 1 {
                Some(transform)
            } else {
                None
            }
        })
        .unwrap();
    hover_rectangle_transform.translation.x = new_tile_world_pos.x;
    hover_rectangle_transform.translation.y = new_tile_world_pos.y;
}
