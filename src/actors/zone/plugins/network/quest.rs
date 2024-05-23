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

use atlas::{dialogStructure, oaDialogChoice, oaDialogNode, oaDialogQuestPrototype, oaQuestTemplate, AvatarId, CPkt, CPktStream_166_2, NpcBaseComponent, NpcBaseParams, OaDialogChoiceEmoteIndex, OaPktQuestRequestRequest, ParamBox, PlayerComponent, PlayerParams, Uuid};
use bevy_ecs::{component::Component, entity::Entity, event::Event, query::{Added, With}, system::{Commands, In, Query, Res}};

use crate::actors::{AvatarComponent, AvatarIdToEntityLookup, CurrentTarget, DialogChoice, DIALOGS};

use super::PlayerController;

#[derive(Component)]
pub struct QuestProgress {
    available: Vec<i32>,
    in_progress: Vec<i32>,
    completed: Vec<i32>,
}

pub fn initialize_quest_progress(
    query: Query<Entity, Added<PlayerComponent>>,
    mut cmds: Commands,
) {
    for player in query.iter() {
        cmds.entity(player).insert(QuestProgress {
            available: vec![],
            in_progress: vec![],
            completed: vec![]
        });
    }
}

pub fn handle_quest_request(
    In(pkt): In<CPkt>,
    mut player: Query<(&PlayerController, &mut ParamBox, &mut QuestProgress), With<PlayerComponent>>,
    avatar_id_lookup: Res<AvatarIdToEntityLookup>,
) {
    if 
        let CPkt::oaPktQuestRequest(pkt) = pkt &&
        let Some(player_ent) = avatar_id_lookup.get(&pkt.player) &&
        let Ok((controller, mut params, mut progress)) = player.get_mut(*player_ent)
    {
        let params = params.get_impl_mut::<dyn PlayerParams>().unwrap();

        match pkt.request {
            OaPktQuestRequestRequest::Accept => {
                progress.in_progress.push(pkt.quest_id);

                let mut quest_track = params.my_quest_track()
                    .to_vec();

                quest_track.push(pkt.quest_id);

                params.set_my_quest_track(quest_track);
            },
            OaPktQuestRequestRequest::Abandon => {

            },
            OaPktQuestRequestRequest::Request => unreachable!(),
            OaPktQuestRequestRequest::QueryActive => {

            },
        };
    }
}

