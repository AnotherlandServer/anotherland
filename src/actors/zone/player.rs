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

pub enum PlayerSpawnMode {
    LoginFirstTime, // 1
    LoginNormal, // 2
    TravelDirect, // 3
    TravelPortal, // 4
    TravelCarrier, // 5
    TravelPoint, // 6   
}

impl Into<i32> for PlayerSpawnMode {
    fn into(self) -> i32 {
        match self {
            Self::LoginFirstTime => 1,
            Self::LoginNormal => 2,
            Self::TravelDirect => 3,
            Self::TravelPortal => 4,
            Self::TravelCarrier => 5,
            Self::TravelPoint => 6,
        }
    }
}