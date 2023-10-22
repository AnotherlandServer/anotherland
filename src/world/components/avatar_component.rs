use atlas::AvatarId;
use glam::Vec3;

#[derive(Clone, Debug)]
pub struct AvatarComponent {
    pub id: AvatarId,
    pub name: String,
    pub vel: Vec3,
}