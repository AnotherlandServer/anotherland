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

use std::sync::Arc;

use bevy::{ecs::{component::Component, entity::Entity, message::Message, resource::Resource}, platform::collections::{hash_map, HashMap}};
use mlua::Table;
use realm_api::QuestTemplate;

pub struct Quest {
    pub id: i32,
    pub obj: Table,
    pub template: Arc<QuestTemplate>,
}

#[derive(Clone, Copy)]
pub enum QuestState {
    Abandoned,
    Accepted,
    Failed,
    Completed,
    Finished,
}

#[derive(Message)]
pub struct QuestStateUpdated {
    pub player: Entity,
    pub quest_id: i32,
    pub state: QuestState,
}

#[derive(Message)]
pub struct AcceptQuest {
    pub player: Entity,
    pub quest_id: i32,
}

#[derive(Message)]
pub struct AbandonQuest {
    pub player: Entity,
    pub quest_id: i32,
}

#[derive(Message)]
pub struct FailQuest {
    pub player: Entity,
    pub quest_id: i32,
}

#[derive(Message)]
pub struct ReturnQuest {
    pub player: Entity,
    pub quest_id: i32,
}

#[derive(Component)]
pub struct AttachedQuest { 
    pub(super) quest_id: i32 
}

#[derive(Component)]
pub struct QuestAvailable;

#[derive(Component)]
pub struct QuestPlayer(pub Entity);

#[derive(Component)]
pub struct AutoReturnQuest;

#[derive(Component)]
pub struct QuestStatePending;

#[derive(Message)]
pub struct UpdateAvailableQuests(pub Entity);

#[derive(Resource, Default)]
pub struct Quests(HashMap<i32, Quest>);

impl Quests {
    pub fn new(quests: HashMap<i32, Quest>) -> Self {
        Self(quests)
    }

    pub fn get(&self, id: &i32) -> Option<&Quest> {
        self.0.get(id)
    }

    pub fn update(&mut self, quest: Quest) {
        self.0.insert(quest.template.id, quest);
    }

    pub fn iter(&self) -> hash_map::Iter<'_, i32, Quest> {
        self.0.iter()
    }

    pub fn values(&self) -> hash_map::Values<'_, i32, Quest> {
        self.0.values()
    }
}

#[derive(Component)]
pub struct ActiveQuest {
    pub obj: Table,
    pub template: Arc<QuestTemplate>,
}



