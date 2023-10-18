use std::{ops::{Deref, DerefMut}, sync::Arc, cell::RefCell};

use atlas::{ParamClassContainer, Uuid};
use glam::{Vec3, Vec4};
use tokio::sync::RwLock;

use crate::db::Character;

use super::*;

pub trait AvatarBehaviour {
    fn name(&self) -> &String;

    fn position(&self) -> Vec3;
    fn rotation(&self) -> Vec3;
    fn velocity(&self) -> Vec3;

    fn zone_guid(&self) -> Uuid;

    fn params(&self) -> ParamClassContainer;

    fn set_position(&mut self, pos: Vec3);
    fn set_rotation(&mut self, rot: Vec3);
    fn set_velocity(&mut self, velocity: Vec3);

    fn tick(&mut self, delta: f32) {}
}

pub enum Avatar {
    Player(Box<PlayerAvatar>),
    Npc(Box<NpcAvatar>),
    Structure(Box<StructureAvatar>),
    Portal(Box<PortalAvatar>),
    StartingPoint(Box<StartingPointAvatar>),
    Trigger(Box<TriggerAvatar>),
    SpawnNode(Box<SpawnNodeAvatar>),
}

impl Deref for Avatar {
    type Target = dyn AvatarBehaviour;

    fn deref(&self) -> &Self::Target {
        match self {
            Avatar::Player(avatar) => avatar.as_ref() as &dyn AvatarBehaviour,
            Avatar::Npc(avatar) => avatar.as_ref() as &dyn AvatarBehaviour,
            Avatar::Portal(avatar) => avatar.as_ref() as &dyn AvatarBehaviour,
            Avatar::SpawnNode(avatar) => avatar.as_ref() as &dyn AvatarBehaviour,
            Avatar::StartingPoint(avatar) => avatar.as_ref() as &dyn AvatarBehaviour,
            Avatar::Structure(avatar) => avatar.as_ref() as &dyn AvatarBehaviour,
            Avatar::Trigger(avatar) => avatar.as_ref() as &dyn AvatarBehaviour,
        }
    }
}

impl DerefMut for Avatar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Avatar::Player(avatar) => avatar.as_mut() as &mut dyn AvatarBehaviour,
            Avatar::Npc(avatar) => avatar.as_mut() as &mut dyn AvatarBehaviour,
            Avatar::Portal(avatar) => avatar.as_mut() as &mut dyn AvatarBehaviour,
            Avatar::SpawnNode(avatar) => avatar.as_mut() as &mut dyn AvatarBehaviour,
            Avatar::StartingPoint(avatar) => avatar.as_mut() as &mut dyn AvatarBehaviour,
            Avatar::Structure(avatar) => avatar.as_mut() as &mut dyn AvatarBehaviour,
            Avatar::Trigger(avatar) => avatar.as_mut() as &mut dyn AvatarBehaviour,
        }
    }
}
