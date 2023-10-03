use super::{Position, AvatarId};

pub enum WorldAction {
    MoveAvatar { id: AvatarId, pos: Position }
}

pub struct World {

}

impl World {
    pub fn spawn_avatar() -> AvatarId {
        todo!()
    }

    pub fn tick(delta: f32) {

    }
}