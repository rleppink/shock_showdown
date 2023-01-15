use crate::prelude::*;

mod hover_rectangle;
mod move_player;
mod pick_up_or_throw;
mod spawn;
mod throw;
mod update_tile_pos;

pub use hover_rectangle::*;
pub use move_player::*;
pub use pick_up_or_throw::*;
pub use spawn::*;
pub use throw::*;
pub use update_tile_pos::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct LastDirection(pub Vec2);

#[derive(Component)]
pub struct Carried(ObjectType);

#[derive(Component)]
pub struct HoverRectangle;

#[derive(Debug)]
pub struct ThrowEvent {
    player_entity: Entity,
    origin_tile: TilePos,
    direction: IntVec2,
}
