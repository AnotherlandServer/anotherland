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

use bevy::{ecs::{component::Component, entity::Entity, message::Message, resource::Resource}, platform::collections::HashMap};
use mlua::Table;
use realm_api::WorldDef;

pub struct Quest {
    pub(super) table: Table,
    pub(super) id: i32,
    pub(super) owned: bool,
    pub(super) world_def: Arc<WorldDef>,
}

#[derive(Resource, Default)]
pub struct QuestRegistry(pub HashMap<i32, Arc<Quest>>);

#[derive(Clone, Copy)]
pub enum QuestState {
    Available,
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

pub struct QuestProgress {
    pub(super) template: Arc<Quest>,
    pub(super) state: Option<realm_api::QuestState>,
}

#[derive(Component)]
pub struct AttachedQuest { 
    pub(super) quest_id: i32 
}

#[derive(Component)]
pub struct QuestAvailable;

#[derive(Component)]
pub struct QuestPlayer(pub Entity);

