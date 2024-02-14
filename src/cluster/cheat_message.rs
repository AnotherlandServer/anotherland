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

use atlas::{AvatarId, NativeParam, ParamError};
use log::warn;

use crate::util::{AnotherlandError, AnotherlandResult};

#[derive(Debug)]
pub enum CheatMessage {
    InstantKill{ instigator: AvatarId, target: AvatarId },
}

impl CheatMessage {
    pub fn from_native(np: NativeParam) -> AnotherlandResult<Self> {
        let mut values = np.to_struct_iter()?;
        let _ = values.next(); // oaCNode
        let command = values.next().ok_or(ParamError(()))?.to_string()?;
        
        match command.as_str() {
            "instantKill" => {
                let _ = values.next().ok_or(ParamError(()))?.to_string()?;
                let instigator: u64  = values.next().ok_or(ParamError(()))?.to_string()?[1..].parse()
                    .map_err(|_| AnotherlandError::app_err("Invalid avatar id"))?;
                let target: u64 = values.next().ok_or(ParamError(()))?.to_string()?[1..].parse()
                    .map_err(|_| AnotherlandError::app_err("Invalid avatar id"))?;

                Ok(CheatMessage::InstantKill { instigator: instigator.into(), target: target.into() })
            },
            _ => Err(AnotherlandError::app_err(&format!("Unknown cheat command: {}", command)))
        }
    }
}
