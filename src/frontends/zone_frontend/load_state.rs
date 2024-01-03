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

#[derive(Clone, PartialEq, Eq)]
pub(super) enum ClientLoadState {
    EarlyLoadSequence,
    RequestAvatarStream,
    StreamedAvatars,
    RequestSpawn,
    Spawned
}

impl Into<u32> for ClientLoadState {
    fn into(self) -> u32 {
        match self {
            ClientLoadState::EarlyLoadSequence => 0,
            ClientLoadState::RequestAvatarStream => 5,
            ClientLoadState::StreamedAvatars => 6,
            ClientLoadState::RequestSpawn => 7,
            ClientLoadState::Spawned => 8,
        }
    }
}