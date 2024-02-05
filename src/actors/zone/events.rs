// Copyright (C) 2023 AnotherlandServer
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use atlas::{AvatarId, ParamBox, ParamSetBox, Physics};
use glam::{Quat, Vec3};

use super::ProximityChatRange;

pub enum ZoneEvent {
    // avatar events
    AvatarSpawned { avatar_id: AvatarId, params: ParamBox },
    AvatarUpdated { avatar_id: AvatarId, params: ParamSetBox },
    AvatarMoved { avatar_id: AvatarId, movement: Movement },
    AvatarDespawned { avatar_id: AvatarId },
    CombatHpUpdate { avatar_id: AvatarId, hp: i32 },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PhysicsState {
    None,
    Walking,
    Falling,
    Swimming,
    Flying,
    Rotating,
    Projectile,
    Interpolating,
    Spider,
    Ladder,
    RigidBody,
    SoftBody,
    Unused,
    Custom,
    SplineSurfing,
}

impl From<PhysicsState> for Physics {
    fn from(val: PhysicsState) -> Self {
        Physics {
            state: match val {
                PhysicsState::None => atlas::PhysicsState::None,
                PhysicsState::Walking => atlas::PhysicsState::Walking,
                PhysicsState::Falling => atlas::PhysicsState::Falling,
                PhysicsState::Swimming => atlas::PhysicsState::Swimming,
                PhysicsState::Flying => atlas::PhysicsState::Flying,
                PhysicsState::Projectile => atlas::PhysicsState::Projectile,
                PhysicsState::Rotating => atlas::PhysicsState::Rotating,
                PhysicsState::Interpolating => atlas::PhysicsState::Interpolating,
                PhysicsState::Spider => atlas::PhysicsState::Spider,
                PhysicsState::Ladder => atlas::PhysicsState::Ladder,
                PhysicsState::RigidBody => atlas::PhysicsState::RigidBody,
                PhysicsState::SoftBody => atlas::PhysicsState::SoftBody,
                PhysicsState::Unused => atlas::PhysicsState::Unused,
                PhysicsState::Custom => atlas::PhysicsState::Custom,
                PhysicsState::SplineSurfing => atlas::PhysicsState::SplineSurfing,
            }
        }
    }
}

impl From<Physics> for PhysicsState {
    fn from(value: Physics) -> Self {
        match value.state {
            atlas::PhysicsState::None => Self::None,
            atlas::PhysicsState::Walking => Self::Walking,
            atlas::PhysicsState::Falling => Self::Falling,
            atlas::PhysicsState::Swimming => Self::Swimming,
            atlas::PhysicsState::Flying => Self::Flying,
            atlas::PhysicsState::Projectile => Self::Projectile,
            atlas::PhysicsState::Rotating => Self::Rotating,
            atlas::PhysicsState::Interpolating => Self::Interpolating,
            atlas::PhysicsState::Spider => Self::Spider,
            atlas::PhysicsState::Ladder => Self::Ladder,
            atlas::PhysicsState::RigidBody => Self::RigidBody,
            atlas::PhysicsState::SoftBody => Self::SoftBody,
            atlas::PhysicsState::Unused => Self::Unused,
            atlas::PhysicsState::Custom => Self::Custom,
            atlas::PhysicsState::SplineSurfing => Self::SplineSurfing,
        }
    }
}

pub struct Movement {
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
    pub physics_state: PhysicsState,
    pub mover_key: u16,
    pub seconds: f64
}