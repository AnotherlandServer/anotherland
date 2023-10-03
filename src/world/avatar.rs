use glam::{Vec3, Vec4};

use crate::db::Character;

pub type AvatarId = u64;

pub struct Position {
    pub position: Vec3,
    pub rotation: Vec4,
}

pub trait Avatar {
    fn position(&self) -> Position;
    fn update_position(&mut self, pos: &Position);
}