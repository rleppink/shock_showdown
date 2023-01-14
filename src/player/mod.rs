use crate::prelude::*;

mod hover_rectangle;
mod move_player;
mod pick_up_or_throw;
mod spawn;
mod throw;

pub use hover_rectangle::*;
pub use move_player::*;
pub use pick_up_or_throw::*;
pub use spawn::*;
pub use throw::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct LastDirection(pub Vec2);

#[derive(Component)]
pub struct Carried(ObjectType);

#[derive(Component)]
pub struct HoverRectangle;

#[derive(Debug)]
pub struct ThrowEvent(Entity, TilePos);
