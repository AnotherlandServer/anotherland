use atlas::AvatarId;
use glam::Vec3;

pub enum AvatarEvent {
    Move { pos: Vec3, rot: Vec3, vel: Vec3 },
    Spawn { avatar_id: AvatarId, pos: Vec3 },
    Despawn { avatar_id: AvatarId },
}