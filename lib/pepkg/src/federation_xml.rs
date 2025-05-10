// Copyright (C) 2025 AnotherlandServer
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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "federation")]
pub struct Federation {
    #[serde(rename = "@majorRelease")]
    pub major_release: u32,

    #[serde(rename = "@minorRelease")]
    pub minor_release: u32,

    #[serde(rename = "@startX")]
    pub start_x: i32,

    #[serde(rename = "@startY")]
    pub start_y: i32,

    #[serde(rename = "@width")]
    pub width: i32,

    #[serde(rename = "@height")]
    pub height: i32,

    #[serde(rename = "@tileSize")]
    pub tile_size: i32,

    #[serde(rename = "@overlap")]
    pub overlap: i32,

    #[serde(rename = "@translatesBySectionID")]
    pub translates_by_section_id: bool,
}