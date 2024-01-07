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

use atlas::{AvatarId, ParamClassContainer, OaPktMoveManagerPosUpdatePhysicsState};
use glam::{Quat, Vec3};

pub enum ZoneEvent {
    AvatarSpawned { avatar_id: AvatarId, params: ParamClassContainer },
    AvatarUpdated { avatar_id: AvatarId, params: ParamClassContainer },
    AvatarMoved { avatar_id: AvatarId, movement: Movement },
    AvatarDespawned { avatar_id: AvatarId }
}

#[derive(Clone, Copy)]
pub enum PhysicsState {
    Unknown0,
    Standing,
    Falliing,
    Unknown224,
}

impl Into<OaPktMoveManagerPosUpdatePhysicsState> for PhysicsState {
    fn into(self) -> OaPktMoveManagerPosUpdatePhysicsState {
        match self {
            Self::Unknown0 => OaPktMoveManagerPosUpdatePhysicsState::Unknown0,
            Self::Standing => OaPktMoveManagerPosUpdatePhysicsState::Standing,
            Self::Falliing => OaPktMoveManagerPosUpdatePhysicsState::Falling,
            Self::Unknown224 => OaPktMoveManagerPosUpdatePhysicsState::Unknown224,
        }
    }
}

impl From<OaPktMoveManagerPosUpdatePhysicsState> for PhysicsState {
    fn from(value: OaPktMoveManagerPosUpdatePhysicsState) -> Self {
        match value {
            OaPktMoveManagerPosUpdatePhysicsState::Unknown0 => Self::Unknown0,
            OaPktMoveManagerPosUpdatePhysicsState::Standing => Self::Standing,
            OaPktMoveManagerPosUpdatePhysicsState::Falling => Self::Falliing,
            OaPktMoveManagerPosUpdatePhysicsState::Unknown224 => Self::Unknown224,
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