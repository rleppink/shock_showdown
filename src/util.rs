use std::ops::Add;

use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct IntVec2 {
    pub x: i32,
    pub y: i32,
}

impl From<Vec2> for IntVec2 {
    fn from(value: Vec2) -> Self {
        Self {
            x: value.x.round() as i32,
            y: value.y.round() as i32,
        }
    }
}

impl Add<IntVec2> for TilePos {
    type Output = TilePos;

    fn add(self, rhs: IntVec2) -> Self::Output {
        Self {
            x: (self.x as i32 + rhs.x) as u32,
            y: (self.y as i32 + rhs.y) as u32,
        }
    }
}
