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

use crate::{error::Error, get_content_path};

#[derive(Serialize, Deserialize)]
pub struct Skill {
    pub ability: String,
    pub group: String,
    pub level: i32,
    pub upgrade_cost: Option<i32>,
    pub enabled: bool,
    pub stance: i32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CombatStyle {
    pub stances: Vec<String>,
    pub skills: Vec<Skill>,
}

impl CombatStyle {
    pub async fn load(name: &str) -> Result<CombatStyle, Error> {
        let path = get_content_path(format!("combat_styles/{}.yaml", name))?;
        let file = tokio::fs::read(path).await?;
        let style: CombatStyle = serde_yaml::from_slice(&file)?;
        Ok(style)
    }
}