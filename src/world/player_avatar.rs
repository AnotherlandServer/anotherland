use atlas::CParam;

use crate::db::Character;

use super::{Avatar, Position};

pub struct PlayerAvatar {
    character: Character,
}

impl Avatar for PlayerAvatar {
    fn position(&self) -> Position {
        Position {
            position: self.character.data.pos.as_ref().map(|v| v.to_owned().try_into().ok().unwrap_or_default()).unwrap_or_default(),
            rotation: self.character.data.rot.as_ref().map(|v| v.to_owned().try_into().ok().unwrap_or_default()).unwrap_or_default(),
        }
    }

    fn update_position(&mut self, pos: &Position) {
        self.character.data.pos = Some(CParam::Vector3(pos.position));
        self.character.data.rot = Some(CParam::Vector4(pos.rotation));
    }
}