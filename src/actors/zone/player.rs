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

use atlas::Uuid;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PlayerSpawnMode {
    LoginFirstTime, // 1
    LoginNormal, // 2
    TravelDirect, // 3
    TravelPortal(Uuid), // 4
    TravelCarrier, // 5
    TravelPoint, // 6   
}

impl From<PlayerSpawnMode> for i32 {
    fn from(val: PlayerSpawnMode) -> Self {
        match val {
            PlayerSpawnMode::LoginFirstTime => 1,
            PlayerSpawnMode::LoginNormal => 2,
            PlayerSpawnMode::TravelDirect => 3,
            PlayerSpawnMode::TravelPortal(_) => 4,
            PlayerSpawnMode::TravelCarrier => 5,
            PlayerSpawnMode::TravelPoint => 6,
        }
    }
}