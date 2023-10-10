use atlas::{PlayerParam, Uuid, ParamClassContainer};
use glam::{Vec3, Vec4};
use mongodb::Database;

use crate::{db::Character, util::AnotherlandResult};

use super::{AvatarBehaviour, Avatar};

pub struct PlayerAvatar {
    character: Character,
    velocity: Vec3,
}

impl PlayerAvatar {
    pub fn new(character: Character) -> Self {
        Self {
            character,
            velocity: Vec3::default(),
        }
    }

    pub fn player_param(&self) -> &PlayerParam {
        &self.character.data
    }

    pub fn player_param_mut(&mut self) -> &mut PlayerParam {
        &mut self.character.data
    }

    pub async fn save(&mut self, db: Database) -> AnotherlandResult<()> {
        self.character.save(db).await
    }
}

impl AvatarBehaviour for PlayerAvatar {
    fn name(&self) -> &String { &self.character.name }

    fn position(&self) -> Vec3 { self.character.data.pos().map(|v| v.to_owned()).unwrap_or_default() }
    fn rotation(&self) -> Vec3 { self.character.data.rot().map(|v| v.to_owned()).unwrap_or_default() }
    fn velocity(&self) -> Vec3 { self.velocity }

    fn zone_guid(&self) -> Uuid { self.character.data.zone_guid().unwrap().to_owned() }

    fn params(&self) -> ParamClassContainer { ParamClassContainer::Player(self.character.data.clone()) }

    fn set_position(&mut self, pos: Vec3) { self.character.data.set_pos(pos); }
    fn set_rotation(&mut self, rot: Vec3) { self.character.data.set_rot(rot); }
    fn set_velocity(&mut self, velocity: Vec3) { self.velocity = velocity; }
}

impl Into<Avatar> for PlayerAvatar {
    fn into(self) -> Avatar {
        Avatar::Player(Box::new(self))
    }
}
