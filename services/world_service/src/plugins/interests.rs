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

use bevy::{app::{App, Plugin, PreUpdate}, prelude::{Changed, Commands, Component, Entity, IntoSystemConfigs, Query, With, Without}, utils::HashSet};
use bitstream_io::{ByteWriter, LittleEndian};
use log::debug;
use obj_params::{tags::PlayerTag, GameObjectData, NonClientBase, ParamWriter, Player};
use protocol::{oaPktS2XConnectionState, CPktAvatarClientNotify, CPktAvatarUpdate, MoveManagerInit, OaPktS2xconnectionStateState, Physics};
use toolkit::types::UUID_NIL;

use super::{AvatarInfo, ConnectionState, CurrentState, Movement, PlayerController};

pub struct InterestsPlugin;

impl Plugin for InterestsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, 
            (
                    enable_player_interest_building,
                    update_interest_list
                ).chain()
            );
    }
}

#[derive(Component)]
pub struct EnabledInGame;

#[derive(Component)]
pub struct Interests {
    interests: HashSet<Entity>,
}

impl Interests {
    pub fn contains(&self, ent: &Entity) -> bool { self.interests.contains(ent) }
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
                    interests: HashSet::new(),
                },
            ));
        }
    }
}

fn update_interest_list(
    mut players: Query<(Entity, &GameObjectData, &Movement, &mut Interests, &PlayerController, &mut CurrentState), (With<PlayerTag>, With<BuildInterestList>)>,
    potential_interests: Query<(Entity, &AvatarInfo, &Movement, &GameObjectData), With<EnabledInGame>>,
) {
    for (player_ent, player, player_pos, mut interests, controller, mut state) in players.iter_mut() {
        let aware_range: f32 = *player.get(Player::AwareRange).unwrap();

        let mut interest_count = 0; // Limit the amount if interests to be sent each tick

        // determine interests
        for (interest_ent, interest_info, interest_pos, interest_obj) in potential_interests.iter() {
            // skip over self
            if interest_ent == player_ent { continue; }

            let distance = interest_pos.position.distance(player_pos.position);
            if 
                distance < aware_range ||
                *interest_obj.get::<_, bool>(NonClientBase::AlwaysVisibleToPlayers).unwrap_or(&false)
            {
                if !interests.interests.contains(&interest_ent) && interest_count < 10 {
                    interests.interests.insert(interest_ent);

                    debug!("Sending {}", interest_info.name);

                    let mut data = Vec::new();
                    {
                        let mut writer = ByteWriter::endian(&mut data, LittleEndian);
                        interest_obj.write_to_client(&mut writer).unwrap();
                    }

                    // send avatar to client
                    controller.send_packet(CPktAvatarUpdate {
                        full_update: true,
                        avatar_id: Some(interest_info.id),
                        field_2: Some(false),
                        name: Some(interest_info.name.clone()),
                        class_id: Some(interest_obj.class().id() as u32),
                        field_6: Some(UUID_NIL),
                        params: data.into(),
                        update_source: 0,
                        movement: Some(MoveManagerInit {
                            pos: interest_pos.position.into(),
                            rot: interest_pos.rotation.into(),
                            vel: interest_pos.velocity.into(),
                            physics: Physics {
                                state: interest_pos.mode
                            },
                            version: interest_pos.version,
                            mover_type: interest_pos.mover_type,
                            mover_replication_policy: interest_pos.mover_replication_policy,
                            ..Default::default()
                        }.to_bytes().into()),
                        ..Default::default()
                    });

                    interest_count += 1;
                }
            } else if interests.interests.contains(&interest_ent) {
                interests.interests.remove(&interest_ent);

                // notify client
                controller.send_packet(CPktAvatarClientNotify {
                    avatar_id: interest_info.id,
                    ..Default::default()
                });
            }
        }

        if matches!(state.state, ConnectionState::WaitingForInitialInterests) {
            debug!("Initial interests loaded for player {}", player_ent);
            state.state = ConnectionState::ReceivedInitialInterests;

            controller.send_packet(oaPktS2XConnectionState {
                state: state.state.into(),
                ..Default::default()
            });
        }
    }
}