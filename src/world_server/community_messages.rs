use atlas::{AvatarId, NativeParam, ParamError};
use log::warn;

use crate::util::AnotherlandResult;

pub enum CommunityMessage {
    SocialTravel{avatar: AvatarId, map: String, travel: bool},
    Unknown_A1{avatar: AvatarId, boolean: bool},
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
                Ok(CommunityMessage::Unknown_A1 { 
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
