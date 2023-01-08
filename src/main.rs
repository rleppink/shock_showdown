use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_turborand::prelude::*;

pub const MAP_TILE_WIDTH: u32 = 17;
pub const MAP_TILE_HEIGHT: u32 = 11;

pub const MAP_SIZE: TilemapSize = TilemapSize {
    x: MAP_TILE_WIDTH,
    y: MAP_TILE_HEIGHT,
};

pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 64., y: 64. };

pub const MAP_TYPE: TilemapType = TilemapType::Square;

mod collision;
mod map_builder;
mod player;

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
        .add_plugin(RngPlugin::new())
        .insert_resource(ClearColor(Color::rgb(0.6, 0.6, 0.6)))
        .add_startup_system(setup_camera)
        .add_startup_system(map_builder::build_tilemap)
        .add_startup_stage_after(
            StartupStage::Startup,
            "player_startup_stage",
            SystemStage::parallel(),
        )
        .add_startup_system_to_stage("player_startup_stage", player::spawn)
        .add_startup_system_to_stage("player_startup_stage", player::spawn_hover_rectangle)
        .add_startup_system_to_stage("player_startup_stage", player::spawn_target_tile)
        .add_system(player::move_player)
        .add_system(player::draw_hover_rectangle)
        .add_system(player::print_target_tile)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.25,
            ..default()
        },
        ..default()
    });
}
