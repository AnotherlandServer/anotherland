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

use obj_params::{GameObjectData, NonClientBase};

use crate::plugins::ContentInfo;

pub trait AvatarSelectorMatcher {
    fn matches(&self, content_info: &ContentInfo, obj: &GameObjectData) -> bool;
}

impl AvatarSelectorMatcher for realm_api::AvatarSelector {
    fn matches(&self, content_info: &ContentInfo, obj: &GameObjectData) -> bool {
        match *self {
            realm_api::AvatarSelector::ContentId(uuid) => content_info.template.id == uuid,
            realm_api::AvatarSelector::InstanceId(uuid) => content_info.placement_id == uuid,
            realm_api::AvatarSelector::QuestTag(tag) => {
                obj.get::<_, Vec<i32>>(NonClientBase::QuestFlags)
                    .unwrap_or(&vec![])
                    .contains(&tag)
            },
            realm_api::AvatarSelector::LootItem(_uuid) => {
                todo!()
            },
            realm_api::AvatarSelector::DialogId(id) => {
                obj.get::<_, Vec<i32>>(NonClientBase::Dialogs)
                    .unwrap_or(&vec![])
                    .contains(&id)
            }
        }
    }
}