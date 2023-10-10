use atlas::{Uuid, StructureParam, ParamClassContainer, ParamClass};
use glam::{Vec3, Vec4};

use crate::db::{Character, Content, StructureContent};

use super::{AvatarBehaviour, Avatar};

pub struct StructureAvatar {
    id: i64,
    guid: Uuid,
    name: String,
    structure: StructureParam,

    velocity: Vec3,
}

impl StructureAvatar {
    pub fn new(content: StructureContent) -> Self {
        let structure = match content.data.as_ref() {
            Some(ParamClassContainer::Structure(npc)) => npc,
            _ => panic!("Tried to create structure avatar from non-structure content data!"),
        };

        Self {
            id: content.id,
            guid: content.guid.clone(),
            name: content.name.clone(),
            structure: structure.clone(),
            velocity: Vec3::default(),
        }
    }
}

impl AvatarBehaviour for StructureAvatar {
    fn name(&self) -> &String { &self.name }

    fn position(&self) -> Vec3 { self.structure.pos::<Vec3>().map(|v| v.to_owned()).unwrap_or_default() }
    fn rotation(&self) -> Vec3 { self.structure.rot::<Vec3>().map(|v| v.to_owned()).unwrap_or_default() }
    fn velocity(&self) -> Vec3 { self.velocity }

    fn zone_guid(&self) -> Uuid { self.structure.zone_guid::<Uuid>().map(|v| v.to_owned()).unwrap_or_default() }

    fn params(&self) -> ParamClassContainer { ParamClassContainer::Structure(self.structure.clone()) }

    fn set_position(&mut self, pos: Vec3) { self.structure.set_pos(pos); }
    fn set_rotation(&mut self, rot: Vec3) { self.structure.set_rot(rot); }
    fn set_velocity(&mut self, velocity: Vec3) { self.velocity = velocity; }
}

impl Into<Avatar> for StructureAvatar {
    fn into(self) -> Avatar {
        Avatar::Structure(Box::new(self))
    }
}
