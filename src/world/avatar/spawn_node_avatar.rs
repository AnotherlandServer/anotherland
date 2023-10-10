use atlas::{Uuid, ParamClassContainer, SpawnNodeParam};
use glam::{Vec3, Vec4};

use crate::db::{Character, Content, StructureContent};

use super::{AvatarBehaviour, Avatar};

pub struct SpawnNodeAvatar {
    id: i64,
    guid: Uuid,
    name: String,
    spawn_node: SpawnNodeParam,

    velocity: Vec3,
}

impl SpawnNodeAvatar {
    pub fn new(content: StructureContent) -> Self {
        let spawn_node = match content.data.as_ref() {
            Some(ParamClassContainer::SpawnNode(npc)) => npc,
            _ => panic!("Tried to create spawnnode avatar from non-spawnnode content data!"),
        };

        Self {
            id: content.id,
            guid: content.guid.clone(),
            name: content.name.clone(),
            spawn_node: spawn_node.clone(),
            velocity: Vec3::default(),
        }
    }
}

impl AvatarBehaviour for SpawnNodeAvatar {
    fn name(&self) -> &String { &self.name }

    fn position(&self) -> Vec3 { self.spawn_node.pos::<Vec3>().map(|v| v.to_owned()).unwrap_or_default() }
    fn rotation(&self) -> Vec3 { self.spawn_node.rot::<Vec3>().map(|v| v.to_owned()).unwrap_or_default() }
    fn velocity(&self) -> Vec3 { self.velocity }

    fn zone_guid(&self) -> Uuid { self.spawn_node.zone_guid::<Uuid>().map(|v| v.to_owned()).unwrap_or_default() }

    fn params(&self) -> ParamClassContainer { ParamClassContainer::SpawnNode(self.spawn_node.clone()) }

    fn set_position(&mut self, pos: Vec3) { self.spawn_node.set_pos(pos); }
    fn set_rotation(&mut self, rot: Vec3) { self.spawn_node.set_rot(rot); }
    fn set_velocity(&mut self, velocity: Vec3) { self.velocity = velocity; }
}

impl Into<Avatar> for SpawnNodeAvatar {
    fn into(self) -> Avatar {
        Avatar::SpawnNode(Box::new(self))
    }
}
