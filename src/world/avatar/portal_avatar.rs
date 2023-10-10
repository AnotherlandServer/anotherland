use atlas::{Uuid, PortalParam, ParamClassContainer};
use glam::{Vec3, Vec4};

use crate::db::{Character, Content, StructureContent};

use super::{AvatarBehaviour, Avatar};

pub struct PortalAvatar {
    id: i64,
    guid: Uuid,
    name: String,
    portal: PortalParam,

    velocity: Vec3,
}

impl PortalAvatar {
    pub fn new(content: StructureContent) -> Self {
        let portal = match content.data.as_ref() {
            Some(ParamClassContainer::Portal(npc)) => npc,
            _ => panic!("Tried to create portal avatar from non-portal content data!"),
        };

        Self {
            id: content.id,
            guid: content.guid.clone(),
            name: content.name.clone(),
            portal: portal.clone(),
            velocity: Vec3::default(),
        }
    }
}

impl AvatarBehaviour for PortalAvatar {
    fn name(&self) -> &String { &self.name }

    fn position(&self) -> Vec3 { self.portal.pos::<Vec3>().map(|v| v.to_owned()).unwrap_or_default() }
    fn rotation(&self) -> Vec3 { self.portal.rot::<Vec3>().map(|v| v.to_owned()).unwrap_or_default() }
    fn velocity(&self) -> Vec3 { self.velocity }

    fn zone_guid(&self) -> Uuid { self.portal.zone_guid::<Uuid>().map(|v| v.to_owned()).unwrap_or_default() }

    fn params(&self) -> ParamClassContainer { ParamClassContainer::Portal(self.portal.clone()) }

    fn set_position(&mut self, pos: Vec3) { self.portal.set_pos(pos); }
    fn set_rotation(&mut self, rot: Vec3) { self.portal.set_rot(rot); }
    fn set_velocity(&mut self, velocity: Vec3) { self.velocity = velocity; }
}

impl Into<Avatar> for PortalAvatar {
    fn into(self) -> Avatar {
        Avatar::Portal(Box::new(self))
    }
}
