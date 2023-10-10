use atlas::{Uuid, ParamClassContainer, TriggerParam};
use glam::{Vec3, Vec4};

use crate::db::{Character, Content, StructureContent};

use super::{AvatarBehaviour, Avatar};

pub struct TriggerAvatar {
    id: i64,
    guid: Uuid,
    name: String,
    trigger: TriggerParam,

    velocity: Vec3,
}

impl TriggerAvatar {
    pub fn new(content: StructureContent) -> Self {
        let trigger = match content.data.as_ref() {
            Some(ParamClassContainer::Trigger(npc)) => npc,
            _ => panic!("Tried to create trigger avatar from non-trigger content data!"),
        };

        Self {
            id: content.id,
            guid: content.guid.clone(),
            name: content.name.clone(),
            trigger: trigger.clone(),
            velocity: Vec3::default(),
        }
    }
}

impl AvatarBehaviour for TriggerAvatar {
    fn name(&self) -> &String { &self.name }

    fn position(&self) -> Vec3 { self.trigger.pos::<Vec3>().map(|v| v.to_owned()).unwrap_or_default() }
    fn rotation(&self) -> Vec3 { self.trigger.rot::<Vec3>().map(|v| v.to_owned()).unwrap_or_default() }
    fn velocity(&self) -> Vec3 { self.velocity }

    fn zone_guid(&self) -> Uuid { self.trigger.zone_guid::<Uuid>().map(|v| v.to_owned()).unwrap_or_default() }

    fn params(&self) -> ParamClassContainer { ParamClassContainer::Trigger(self.trigger.clone()) }

    fn set_position(&mut self, pos: Vec3) { self.trigger.set_pos(pos); }
    fn set_rotation(&mut self, rot: Vec3) { self.trigger.set_rot(rot); }
    fn set_velocity(&mut self, velocity: Vec3) { self.velocity = velocity; }
}

impl Into<Avatar> for TriggerAvatar {
    fn into(self) -> Avatar {
        Avatar::Trigger(Box::new(self))
    }
}
