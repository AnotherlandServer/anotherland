// Copyright (C) 2024 AnotherlandServer
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

use log::kv::ToValue;

#[derive(Clone, Copy)]
pub enum ProximityChatRange {
    Say,
    TeamSay,
    Shout,
}

impl ToValue for ProximityChatRange {
    fn to_value(&self) -> log::kv::Value {
        match self {
            Self::Say => "say",
            Self::TeamSay => "teamsay",
            Self::Shout => "shout",
        }.to_value()
    }
}

impl ProximityChatRange {
    // todo: validate those ranges
    pub fn aware_dist(&self) -> f32 {
        match self {
            ProximityChatRange::Say => 1000.0,
            ProximityChatRange::TeamSay => 1000.0,
            ProximityChatRange::Shout => 3900.0,
        }
    }
}