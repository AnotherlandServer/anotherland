use atlas::{Uuid, StartingPointParam, ParamClassContainer};
use glam::{Vec3, Vec4};

use crate::db::{Character, Content, StructureContent};

use super::{AvatarBehaviour, Avatar};

pub struct StartingPointAvatar {
    id: i64,
    guid: Uuid,
    name: String,
    starting_point: StartingPointParam,

    velocity: Vec3,
}

impl StartingPointAvatar {
    pub fn new(content: StructureContent) -> Self {
        let starting_point = match content.data.as_ref() {
            Some(ParamClassContainer::StartingPoint(npc)) => npc,
            _ => panic!("Tried to create startingpoint avatar from non-startingpoint content data!"),
        };

        Self {
            id: content.id,
            guid: content.guid.clone(),
            name: content.name.clone(),
            starting_point: starting_point.clone(),
            velocity: Vec3::default(),
        }
    }
}

impl AvatarBehaviour for StartingPointAvatar {
    fn name(&self) -> &String { &self.name }

    fn position(&self) -> Vec3 { self.starting_point.pos::<Vec3>().map(|v| v.to_owned()).unwrap_or_default() }
    fn rotation(&self) -> Vec3 { self.starting_point.rot::<Vec3>().map(|v| v.to_owned()).unwrap_or_default() }
    fn velocity(&self) -> Vec3 { self.velocity }

    fn zone_guid(&self) -> Uuid { self.starting_point.zone_guid::<Uuid>().map(|v| v.to_owned()).unwrap_or_default() }

    fn params(&self) -> ParamClassContainer { ParamClassContainer::StartingPoint(self.starting_point.clone()) }

    fn set_position(&mut self, pos: Vec3) { self.starting_point.set_pos(pos); }
    fn set_rotation(&mut self, rot: Vec3) { self.starting_point.set_rot(rot); }
    fn set_velocity(&mut self, velocity: Vec3) { self.velocity = velocity; }
}

impl Into<Avatar> for StartingPointAvatar {
    fn into(self) -> Avatar {
        Avatar::StartingPoint(Box::new(self))
    }
}
