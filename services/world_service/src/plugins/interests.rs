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

use std::time::Duration;

use bevy::{app::{App, Plugin, PostUpdate, PreUpdate, Update}, prelude::{Changed, Commands, Component, Entity, IntoSystemConfigs, Or, Query, With, Without}, time::common_conditions::on_timer, utils::{HashMap, HashSet}};
use bitstream_io::{ByteWriter, LittleEndian};
use log::debug;
use obj_params::{tags::{NonClientBaseTag, PlayerTag}, Class, GameObjectData, NonClientBase, ParamWriter, Player};
use protocol::{oaPktS2XConnectionState, CPktAvatarClientNotify, CPktAvatarUpdate, MoveManagerInit, OaPktS2xconnectionStateState, Physics};
use toolkit::types::UUID_NIL;

use super::{Active, AvatarInfo, ConnectionState, CurrentState, Movement, PlayerController, QuestEntity, QuestLog};

pub struct InterestsPlugin;

impl Plugin for InterestsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            (
                (
                    enable_player_interest_building,
                    update_interest_list
                )
                .chain()
                .run_if(on_timer(Duration::from_millis(250))),
                transmit_entities_to_players
                    .after(update_interest_list)
                    .run_if(on_timer(Duration::from_secs(1)))
            ));
    }
}

#[derive(Default)]
pub enum InterestState {
    #[default]
    Initial,
    Transmitted,
}

#[derive(Component)]
pub struct Interests {
    interests: HashMap<Entity, InterestState>,
}

impl Interests {
    pub fn contains(&self, ent: &Entity) -> bool { self.interests.contains_key(ent) }
}

#[derive(Component)]
pub struct BuildInterestList;

fn enable_player_interest_building(
    players: Query<(Entity, &CurrentState), (Changed<CurrentState>, Without<BuildInterestList>)>,
    mut commands: Commands,
) {
    for (player_ent, state) in players.iter() {
        if matches!(state.state, ConnectionState::WaitingForInitialInterests) {
            commands.entity(player_ent).insert((
                BuildInterestList,
                Interests {
                    interests: HashMap::new(),
                },
            ));
        }
    }
}

fn transmit_entities_to_players(
    mut players: Query<(&PlayerController, &mut Interests, &mut CurrentState)>,
    objects: Query<(&AvatarInfo, &Movement, &GameObjectData), With<Active>>,
) {
    for (controller, mut interests, mut state) in players.iter_mut() {
        let mut transmit_order = vec![];

        for (ent, state) in interests.interests.iter() {
            if 
                matches!(state, InterestState::Initial) &&
                let Ok((_, _, obj)) = objects.get(*ent)
            {
                let priority = *obj.get(NonClientBase::Priority).unwrap_or(&999.0);
                transmit_order.push((*ent, priority));
            }
        }

        transmit_order.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
        transmit_order.truncate(10);

        for (ent, _) in &transmit_order {
            if let Ok((avatar, movement, object)) = objects.get(*ent) {
                let mut data = Vec::new();
                {
                    let mut writer = ByteWriter::endian(&mut data, LittleEndian);
                    object.write_to_client(&mut writer).unwrap();
                }

                // send avatar to client
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
                        ..Default::default()
                    }.to_bytes().into()),
                    ..Default::default()
                });

                interests.interests.insert(*ent, InterestState::Transmitted);
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

fn update_interest_list(
    mut players: Query<(Entity, &GameObjectData, &Movement, &mut Interests, &PlayerController, &QuestLog), (With<PlayerTag>, With<BuildInterestList>)>,
    potential_interests: Query<(Entity, &AvatarInfo, &Movement, &GameObjectData, Option<&QuestEntity>), (With<Active>, Or<(With<PlayerTag>, With<NonClientBaseTag>)>)>,
) {
    for (player_ent, player, player_pos, mut interests, controller, quest_log) in players.iter_mut() {
        let aware_range: f32 = *player.get(Player::AwareRange).unwrap();

        // determine interests
        for (interest_ent, interest_info, interest_pos, interest_obj, quest_ent) in potential_interests.iter() {
            // skip over self
            if interest_ent == player_ent { continue; }

            let distance = interest_pos.position.distance(player_pos.position);
            if 
                (
                    distance < aware_range ||
                    *interest_obj.get::<_, bool>(NonClientBase::AlwaysVisibleToPlayers).unwrap_or(&false)
                ) &&
                !interest_obj.get::<_, bool>(NonClientBase::HiddenFromClients).unwrap_or(&false) &&
                (quest_ent.is_none() || quest_ent.unwrap().is_visible(quest_log))
            {
                interests.interests.entry(interest_ent).or_insert(InterestState::Initial);
            } else if let Some(state) = interests.interests.remove(&interest_ent) {
                // notify client about a removed interest
                if matches!(state, InterestState::Transmitted) {
                    controller.send_packet(CPktAvatarClientNotify {
                        avatar_id: interest_info.id,
                        ..Default::default()
                    });
                }
            }
        }
    }
}