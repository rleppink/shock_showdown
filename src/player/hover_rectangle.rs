use crate::prelude::*;

use super::{HoverRectangle, Player};

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
