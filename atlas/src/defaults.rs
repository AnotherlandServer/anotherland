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

use crate::{NetworkVec3, NetworkVec4, PositionUpdate};

use super::{oaCharacterList, oaCharacter, oaFriendList, oaFriendInfo, Uuid};

impl Default for oaCharacterList {
    fn default() -> Self {
        Self {
            count: 0,
            characters: Vec::new(),
        }
    }
}

impl Default for oaCharacter {
    fn default() -> Self {
        Self {
            id: 0,
            field_5: 0,
            length: 0,
            name: String::default(),
            params: Vec::default(),
            world_id: 0,
        }
    }
}

impl Default for oaFriendList {
    fn default() -> Self {
        Self {
            count: 0,
            friends: Vec::default(),
        }
    }
}

impl Default for oaFriendInfo {
    fn default() -> Self {
        Self {
            field_0: 0,
            field_1: 0,
            field_2: String::default(),
            field_3: 0,
            field_4: 0,
            field_5: 0,
            field_6: 0,
            field_7: false,
            field_8: Uuid::default(),
        }
    }
}

impl Default for NetworkVec3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Default for NetworkVec4 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}

impl Default for PositionUpdate {
    fn default() -> Self {
        Self {
            pos: NetworkVec3::default(),
            rot: NetworkVec4::default(),
            vel: NetworkVec3::default(),
            field_3: 0,
            field_4: 0,
            field_5: 0,
            field_6: 0,
        }
    }
}
