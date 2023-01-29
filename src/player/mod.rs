use std::collections::HashMap;

use crate::prelude::*;

mod hover_rectangle;
mod keymaps;
mod move_player;
mod pick_up_or_throw;
mod spawn;
mod throw;
mod update_tile_pos;

pub use hover_rectangle::*;
pub use keymaps::*;
pub use move_player::*;
pub use pick_up_or_throw::*;
pub use spawn::*;
pub use throw::*;
pub use update_tile_pos::*;

#[derive(Component)]
pub struct Player(pub usize);

#[derive(Component)]
pub struct LastDirection(pub Vec2);

#[derive(Component)]
pub struct Carried(ObjectType);

#[derive(Component)]
pub struct HoverRectangle(pub usize);

#[derive(Debug)]
pub struct ThrowEvent {
    player_entity: Entity,
    origin_tile: TilePos,
    direction: IntVec2,
}

#[derive(Resource)]
pub struct PlayerKeyMaps(pub Vec<PlayerKeyMap>);

pub struct PlayerKeyMap {
    pub player_number: usize,
    pub key_map: HashMap<KeyCode, Action>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    PickUpThrow,
}
