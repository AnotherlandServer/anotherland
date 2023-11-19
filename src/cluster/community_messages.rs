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

use atlas::{AvatarId, NativeParam, ParamError};
use log::warn;

use crate::util::AnotherlandResult;

pub enum CommunityMessage {
    SocialTravel{avatar: AvatarId, map: String, travel: bool},
    UnknownA1{avatar: AvatarId, boolean: bool},
}

impl CommunityMessage {
    pub fn from_native(np: NativeParam) -> AnotherlandResult<Self> {
        let mut values = np.to_struct_iter()?;
        let msgtype = values.next().ok_or(ParamError(()))?.to_i32()?;

        match msgtype {
            0xb3 => {
                Ok(CommunityMessage::SocialTravel { 
                    avatar: values.next().ok_or(ParamError(()))?.to_avatar_id()?, 
                    map: values.next().ok_or(ParamError(()))?.to_string()?, 
                    travel: values.next().ok_or(ParamError(()))?.to_bool()?, 
                })
            },
            0xa1 => {
                Ok(CommunityMessage::UnknownA1 { 
                    avatar: values.next().ok_or(ParamError(()))?.to_avatar_id()?, 
                    boolean: values.next().ok_or(ParamError(()))?.to_bool()? 
                })
            }
            _ => {
                warn!("Unknown community message id: {}", msgtype);
                todo!();
            }
        }
    }
}
