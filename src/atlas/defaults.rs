use super::{oaCharacterList, oaCharacter, oaFriendList, oaFriendInfo, Uuid};

impl Default for oaCharacterList {
    fn default() -> Self {
        Self {
            count: 0,
            characters: Vec::new(),
        }
    }
}

impl Default for oaCharacter {
    fn default() -> Self {
        Self {
            field_0: 0,
            field_5: 0,
            length: 0,
            name: String::default(),
            params: Vec::default(),
            world_id: 0,
        }
    }
}

impl Default for oaFriendList {
    fn default() -> Self {
        Self {
            count: 0,
            friends: Vec::default(),
        }
    }
}

impl Default for oaFriendInfo {
    fn default() -> Self {
        Self {
            field_0: 0,
            field_1: 0,
            field_2: String::default(),
            field_3: 0,
            field_4: 0,
            field_5: 0,
            field_6: 0,
            field_7: false,
            field_8: Uuid::default(),
        }
    }
}