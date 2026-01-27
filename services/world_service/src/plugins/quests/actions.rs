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

use bevy::ecs::{entity::Entity, message::{Message, MessageReader}, query::With, system::{Commands, In, Query, Res}};
use log::warn;
use obj_params::tags::PlayerTag;
use protocol::{OaPktRequestQuestActionKind, oaPktRequestQuestAction};
use scripting::{EntityScriptCommandsExt, ScriptObject};

use crate::plugins::{AvatarIdManager, DialogueState};

#[derive(Message)]
pub struct RequestNextQuest {
    pub player: Entity,
}

pub(super) fn handle_quest_action_request(
    In((ent, pkt)): In<(Entity, oaPktRequestQuestAction)>,
    avatar_id_manager: Res<AvatarIdManager>,
    objects: Query<&ScriptObject>,
    mut commands: Commands,
) {
    if 
        let Some(target_ent) = avatar_id_manager.resolve_avatar_id(pkt.target) &&
        let Ok(target) = objects.get(target_ent)
    {
        commands
            .entity(ent)
            .call_named_lua_method("RequestInteraction", (
                if let OaPktRequestQuestActionKind::Interact = pkt.kind {
                    "interact"
                } else {
                    warn!("Unknown quest action kind: {:?}", pkt.kind);
                    "unknown"
                },
                target.object().clone()
            ));
    }    
}

pub(super) fn quest_segue_handler(
    mut events: MessageReader<RequestNextQuest>,
    query: Query<(&ScriptObject, &DialogueState), With<PlayerTag>>,
    mut commands: Commands,
) {
    for &RequestNextQuest { player } in events.read() {
        if let Ok((script_object, dialogue_state)) = query.get(player) {
            commands.entity(dialogue_state.speaker)
                .call_named_lua_method("RequestDialogue", script_object.object().clone());
        }
    }
}