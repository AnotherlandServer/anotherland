use atlas::{NpcOtherlandParam, Uuid, ParamClassContainer};
use glam::{Vec3, Vec4};

use crate::db::{Character, Content, NpcContent};

use super::{AvatarBehaviour, Avatar};

pub struct NpcAvatar {
    id: i64,
    guid: Uuid,
    name: String,
    npc: NpcOtherlandParam,

    velocity: Vec3,
}

impl NpcAvatar {
    pub fn new(content: NpcContent) -> Self {
        let npc = match content.data.as_ref() {
            Some(ParamClassContainer::NpcOtherland(npc)) => npc,
            _ => panic!("Tried to create npc avatar from non-npc content data!"),
        };

        Self {
            id: content.id,
            guid: content.guid.clone(),
            name: content.name.clone(),
            npc: npc.clone(),
            velocity: Vec3::default(),
        }
    }
}

impl AvatarBehaviour for NpcAvatar {
    fn name(&self) -> &String { &self.name }

    fn position(&self) -> Vec3 { self.npc.pos::<Vec3>().map(|v| v.to_owned()).unwrap_or_default() }
    fn rotation(&self) -> Vec3 { self.npc.rot::<Vec3>().map(|v| v.to_owned()).unwrap_or_default() }
    fn velocity(&self) -> Vec3 { self.velocity }

    fn zone_guid(&self) -> Uuid { self.npc.zone_guid::<Uuid>().map(|v| v.to_owned()).unwrap_or_default() }

    fn params(&self) -> ParamClassContainer { ParamClassContainer::NpcOtherland(self.npc.clone()) }

    fn set_position(&mut self, pos: Vec3) { self.npc.set_pos(pos); }
    fn set_rotation(&mut self, rot: Vec3) { self.npc.set_rot(rot); }
    fn set_velocity(&mut self, velocity: Vec3) { self.velocity = velocity; }
}

impl Into<Avatar> for NpcAvatar {
    fn into(self) -> Avatar {
        Avatar::Npc(Box::new(self))
    }
}
