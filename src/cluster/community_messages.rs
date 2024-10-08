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

use crate::util::AnotherlandResult;

#[derive(Debug)]
pub enum CommunityMessage {
    TravelToMap{avatar: AvatarId, map: String, flag: i32},
    LeaveClassTest{avatar: AvatarId, boolean: bool},
    SocialTravel{avatar: AvatarId, map: String, travel: bool},
    UnknownA1{avatar: AvatarId, boolean: bool},
    UnknownA2{avatar: AvatarId, cheat: String},
    Unknown77{avatar: AvatarId},
}

impl CommunityMessage {
    pub fn from_native(np: NativeParam) -> AnotherlandResult<Self> {
        let mut values = np.to_struct_iter()?;
        let msgtype = values.next().ok_or(ParamError(()))?.to_i32()?;

        match msgtype {
            0x31 => {
                Ok(CommunityMessage::TravelToMap { 
                    avatar: values.next().ok_or(ParamError(()))?.to_avatar_id()?, 
                    map: values.next().ok_or(ParamError(()))?.to_string()?, 
                    flag: values.next().ok_or(ParamError(()))?.to_i32()?
                })
            },
            0x35 => {
                Ok(CommunityMessage::LeaveClassTest { 
                    avatar: values.next().ok_or(ParamError(()))?.to_avatar_id()?, 
                    boolean: values.next().ok_or(ParamError(()))?.to_bool()?, 
                })
            },
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
            },
            0xa2 => {
                Ok(CommunityMessage::UnknownA2 { 
                    avatar: values.next().ok_or(ParamError(()))?.to_avatar_id()?, 
                    cheat: values.next().ok_or(ParamError(()))?.to_string()?
                })
            }
            0x77 => {
                Ok(CommunityMessage::Unknown77 { 
                    avatar: values.next().ok_or(ParamError(()))?.to_avatar_id()?
                })
            },
            _ => {
                warn!("Unknown community message id: {}", msgtype);
                todo!();
            }
        }
    }
}
