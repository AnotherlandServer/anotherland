// Copyright (C) 2026 AnotherlandServer
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

use std::time::Duration;

use bevy::{app::{App, Plugin, PreUpdate}, ecs::{message::{Message, MessageReader, MessageWriter}, query::Added, schedule::IntoScheduleConfigs, system::{In, Res}}, platform::collections::HashMap, prelude::{Changed, Commands, Component, Entity, Or, Query, With, Without}, time::common_conditions::on_timer};
use bitstream_io::{ByteWriter, LittleEndian};
use log::debug;
use obj_params::{tags::{NonClientBaseTag, NpcOtherlandTag, PlayerTag}, GameObjectData, NonClientBase, ParamWriter};
use protocol::{oaPktS2XConnectionState, CPktAvatarClientNotify, CPktAvatarUpdate, MoveManagerInit, Physics};
use scripting::{EntityScriptCommandsExt, LuaEntity, ScriptAppExt};
use toolkit::types::{AvatarId, UUID_NIL};
use anyhow::anyhow;

use crate::{error::WorldResult, plugins::{ContentInfo, DebugPlayer, WorldSpace}};

use super::{Active, Avatar, ConnectionState, CurrentState, Movement, PlayerController, QuestVisibility, QuestLog};

pub struct InterestsPlugin;

impl Plugin for InterestsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<InterestAdded>();
        app.add_message::<InterestTransmitted>();
        app.add_message::<InterestRemoved>();

        app.add_systems(PreUpdate, 
            (
                enable_npc_interest_building,
                (
                    enable_player_interest_building,
                    update_interest_list,
                    (notify_interest_added, notify_interest_removed)
                )
                .chain()
                .run_if(on_timer(Duration::from_millis(250))),
                transmit_entities_to_players
                    .after(update_interest_list)
                    .run_if(on_timer(Duration::from_secs(1)))
            ));

        insert_interests_api(app);
    }
}

fn insert_interests_api(app: &mut App) {
    app
        .add_lua_api("interests", "GetInterests",
        |
            In(ent): In<LuaEntity>,
            query: Query<&Interests>,
        | -> WorldResult<Vec<LuaEntity>> {
            let interests = query.get(ent.take())
                .map_err(|_| anyhow!("player not found"))?;

            let mut result = Vec::new();

            for ent in interests.collection().keys() {
                result.push(LuaEntity(*ent));
            }

            Ok(result)
        });
}


#[derive(Default)]
pub enum InterestState {
    #[default]
    Initial,
    Transmitted,
}

#[derive(Component)]
pub struct Interests {
    interests: HashMap<Entity, (AvatarId, InterestState)>,
}

impl Interests {
    pub fn collection(&self) -> &HashMap<Entity, (AvatarId, InterestState)> {
        &self.interests
    }

    pub fn contains(&self, ent: &Entity) -> bool {
        self.interests.contains_key(ent)
    }
}

#[derive(Message)]
pub struct InterestAdded(pub Entity, pub Entity);

#[derive(Message)]
pub struct InterestTransmitted(pub Entity, pub Entity);

#[derive(Message)]
pub struct InterestRemoved(pub Entity, pub Entity);

#[allow(clippy::type_complexity)]
fn enable_player_interest_building(
    players: Query<(Entity, &CurrentState), (Changed<CurrentState>, Without<Interests>)>,
    mut commands: Commands,
) {
    for (player_ent, state) in players.iter() {
        if matches!(state.state, ConnectionState::WaitingForInitialInterests) {
            commands.entity(player_ent).insert((
                Interests {
                    interests: HashMap::new(),
                },
            ));
        }
    }
}

fn enable_npc_interest_building(
    npcs: Query<(Entity, &GameObjectData), Added<NpcOtherlandTag>>,
    mut commands: Commands,
) {
    for (ent, obj) in npcs.iter() {
        if *obj.get::<_, bool>(NonClientBase::GenerateInterestList).unwrap_or(&false) {
            commands.entity(ent).insert((
                Interests {
                    interests: HashMap::new(),
                },
            ));
        }
    }
}

fn transmit_entities_to_players(
    mut players: Query<(Entity, &PlayerController, &mut Interests, &mut CurrentState)>,
    objects: Query<(Option<&ContentInfo>, &Avatar, &Movement, &GameObjectData), With<Active>>,
    mut interest_transmitted_message: MessageWriter<InterestTransmitted>,
) {
    for (player_ent, controller, mut interests, mut state) in players.iter_mut() {
        let mut transmit_order = vec![];

        for (ent, state) in interests.interests.iter() {
            if 
                matches!(state.1, InterestState::Initial) &&
                let Ok((_, _, _, obj)) = objects.get(*ent)
            {
                let priority = *obj.get(NonClientBase::Priority).unwrap_or(&999.0);
                transmit_order.push((*ent, priority));
            }
        }

        transmit_order.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
        transmit_order.truncate(10);

        for (ent, _) in &transmit_order {
            if let Ok((content_info, avatar, movement, object)) = objects.get(*ent) {
                let mut data = Vec::new();
                {
                    let mut writer = ByteWriter::endian(&mut data, LittleEndian);
                    object.write_to_client(&mut writer).unwrap();
                }

                // send avatar to client
                if let Some(content_info) = content_info {
                    // Non-client avatars
                    controller.send_packet(CPktAvatarUpdate {
                        full_update: true,
                        avatar_id: Some(avatar.id),
                        field_2: Some(false),
                        name: Some(avatar.name.clone()),
                        class_id: Some(object.class().id() as u32),
                        field_6: Some(UUID_NIL),
                        params: data.into(),
                        update_source: 0,
                        flags: Some(2 | 4),
                        content_id: Some(content_info.template.id),
                        instance_id: Some(content_info.placement_id),
                        movement: Some(MoveManagerInit {
                            pos: movement.position.into(),
                            rot: movement.rotation.into(),
                            vel: movement.velocity.into(),
                            physics: Physics {
                                state: movement.mode
                            },
                            version: movement.version,
                            mover_type: movement.mover_type,
                            mover_replication_policy: movement.mover_replication_policy,
                            mover_key: movement.mover_key,
                            seconds: movement.seconds,
                            ..Default::default()
                        }.to_bytes().into()),
                        ..Default::default()
                    });
                } else {
                    // Other players
                    controller.send_packet(CPktAvatarUpdate {
                        full_update: true,
                        avatar_id: Some(avatar.id),
                        field_2: Some(false),
                        name: Some(avatar.name.clone()),
                        class_id: Some(object.class().id() as u32),
                        field_6: Some(UUID_NIL),
                        params: data.into(),
                        update_source: 0,
                        movement: Some(MoveManagerInit {
                            pos: movement.position.into(),
                            rot: movement.rotation.into(),
                            vel: movement.velocity.into(),
                            physics: Physics {
                                state: movement.mode
                            },
                            version: movement.version,
                            mover_type: movement.mover_type,
                            mover_replication_policy: movement.mover_replication_policy,
                            mover_key: movement.mover_key,
                            seconds: movement.seconds,
                            ..Default::default()
                        }.to_bytes().into()),
                        ..Default::default()
                    });
                }

                interests.interests.insert(*ent, (avatar.id, InterestState::Transmitted));
                interest_transmitted_message.write(InterestTransmitted(player_ent, *ent));
            }
        }

        if transmit_order.is_empty() && matches!(state.state, ConnectionState::WaitingForInitialInterests) {
            debug!("Initial interests loaded for player {}", controller.avatar_id());
            state.state = ConnectionState::ReceivedInitialInterests;

            controller.send_packet(oaPktS2XConnectionState {
                state: state.state.into(),
                ..Default::default()
            });
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_interest_list(
    world_space: Res<WorldSpace>,
    mut observer: Query<(Entity, &GameObjectData, &Movement, &mut Interests, Option<&QuestVisibility>, Option<&PlayerController>, Option<&QuestLog>), With<Interests>>,
    candidates: Query<(&GameObjectData, Option<&QuestVisibility>, Option<&DebugPlayer>, Option<&QuestLog>), (With<Active>, Or<(With<PlayerTag>, With<NonClientBaseTag>)>)>,
    avatar_info: Query<&Avatar>,
    mut interest_added_message: MessageWriter<InterestAdded>,
    mut interest_removed_message: MessageWriter<InterestRemoved>,
) {
    for (observer_ent, observer_obj, observer_pos, mut interests, observer_visibility, observer_controller, observer_quest_log) in observer.iter_mut() {
        let aware_range: f32 = *observer_obj.get_named("AwareRange").unwrap();

        let found_interests = world_space
            .find_in_range(observer_pos.position, aware_range)
            .into_iter()
            .filter(|&ent| {
                if
                    ent != observer_ent &&
                    let Ok((candidate_obj, candidate_visibility, candiate_debug, candidate_quest_log)) = candidates.get(ent) &&
                    (candiate_debug.is_none() || observer_controller.is_some())
                {
                    if let Some(quest_log) = observer_quest_log {
                        !candidate_obj.get::<_, bool>(NonClientBase::HiddenFromClients).unwrap_or(&false) &&
                        (
                            *candidate_obj.get::<_, bool>(NonClientBase::AlwaysVisibleToPlayers).unwrap_or(&false) ||
                            (candidate_visibility.is_none() || candidate_visibility.unwrap().is_visible(quest_log))
                        )
                    } else if
                        let Some(quest_actor) = observer_visibility &&
                        let Some(quest_ent) = candidate_visibility
                    {
                        quest_actor.is_mutually_visible(quest_ent)
                    } else if 
                        let Some(quest_actor) = observer_visibility &&
                        let Some(candidate_log) = candidate_quest_log
                    {
                        quest_actor.is_visible(candidate_log) ||
                        *observer_obj.get::<_, bool>(NonClientBase::AlwaysVisibleToPlayers).unwrap_or(&false)
                    } else {
                        true
                    }
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();

        // update interests
        for ent in &found_interests {
            if !interests.contains(ent) {
                interests.interests.insert(*ent, (
                    avatar_info.get(*ent).unwrap().id, 
                    InterestState::Initial
                ));

                interest_added_message.write(InterestAdded(observer_ent, *ent));
            }
        }

        // remove interests that are no longer in range
        for ent in interests.interests.keys().cloned().collect::<Vec<_>>() {
            if 
                !found_interests.contains(&ent) &&
                let Some((avatar, state)) = interests.interests.remove(&ent)
            {
                interest_removed_message.write(InterestRemoved(observer_ent, ent));

                if 
                    let Some(controller) = observer_controller &&
                    matches!(state, InterestState::Transmitted)
                {
                    controller.send_packet(CPktAvatarClientNotify {
                        avatar_id: avatar,
                        ..Default::default()
                    });
                }
            }
        }
    }
}

fn notify_interest_added(
    mut messages: MessageReader<InterestAdded>,
    mut commands: Commands,
) {
    for InterestAdded(target, ent) in messages.read() {
        commands
            .entity(*target)
            .fire_lua_event("InterestAdded", LuaEntity(*ent));
    }
}

fn notify_interest_removed(
    mut messages: MessageReader<InterestRemoved>,
    mut commands: Commands,
) {
    for InterestRemoved(target, ent) in messages.read() {
        commands
            .entity(*target)
            .fire_lua_event("InterestRemoved", LuaEntity(*ent));
    }
}