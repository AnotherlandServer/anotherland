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

use bevy::{app::{Last, Plugin, PostUpdate}, ecs::query::{Or, With}, prelude::{Changed, DetectChangesMut, Entity, Query}};
use bitstream_io::{ByteWriter, LittleEndian};
use log::trace;
use obj_params::{GameObjectData, GenericParamSet, ParamFlag, ParamWriter};
use protocol::CPktAvatarUpdate;

use super::{Avatar, Interests, PlayerController, PlayerLocalSets};

pub struct ClientSyncPlugin;

impl Plugin for ClientSyncPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostUpdate, sync_avatar_data);
        app.add_systems(Last, clear_obj_changes);
    }
}

#[allow(clippy::type_complexity)]
fn sync_avatar_data(
    changes: Query<(Entity, &Avatar, &GameObjectData, Option<&PlayerLocalSets>), Or<(Changed<GameObjectData>, Changed<PlayerLocalSets>)>>,
    players: Query<(Entity, &Avatar, Option<&Interests>, &PlayerController)>,
) {
    for (entity, obj_avatar, obj, local_obj) in changes.iter() {
        for (player_ent, player_avatar, interests, controller) in players.iter() {
            if let Some(interests) = interests && interests.contains_key(&entity) {
                let mut param_buffer = Vec::new();
                let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

                let changed_params = if 
                    let Some(local_obj) = local_obj &&
                    let Some(local_set) = local_obj.0.get(&player_ent) 
                {
                    obj.changes()
                        .chain(local_set.changes())
                        .filter(|(attr, _)| !attr.has_flag(&ParamFlag::ClientUnknown) && !attr.has_flag(&ParamFlag::ClientPrivileged))
                        .collect::<Box<dyn GenericParamSet>>()
                } else {
                    obj.changes()
                        .filter(|(attr, _)| !attr.has_flag(&ParamFlag::ClientUnknown) && !attr.has_flag(&ParamFlag::ClientPrivileged))
                        .collect::<Box<dyn GenericParamSet>>()
                };

                if !changed_params.is_empty() {
                    changed_params
                        .write_to_client(&mut writer)
                        .expect("failed to serialize params");

                    trace!("Send param update for avatar: {}", obj_avatar.id);
                    trace!("{changed_params:?}");
    
                    controller.send_packet(CPktAvatarUpdate {
                        full_update: false,
                        avatar_id: Some(obj_avatar.id),
                        update_source: 0,
                        params: param_buffer.into(),
                        ..Default::default()
                    });
                }
            } else if obj_avatar.id == player_avatar.id {
                let mut param_buffer = Vec::new();
                let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

                let changed_params = obj.changes()
                    .filter(|(attr, _)| !attr.has_flag(&ParamFlag::ClientUnknown))
                    .collect::<Box<dyn GenericParamSet>>();

                if !changed_params.is_empty() {
                    changed_params.write_to_privileged_client(&mut writer).expect("failed to serialize params");

                    trace!("Send privileged player update: {}", obj_avatar.id);
                    trace!("{changed_params:?}");

                    controller.send_packet(CPktAvatarUpdate {
                        full_update: false,
                        avatar_id: Some(obj_avatar.id),
                        update_source: 0,
                        params: param_buffer.into(),
                        ..Default::default()
                    });
                }
            }
        }
    }
}

pub fn clear_obj_changes(
    mut changes: Query<&mut GameObjectData, Changed<GameObjectData>>,
    mut player_local_changes: Query<&mut PlayerLocalSets, Changed<PlayerLocalSets>>,
) {
    for mut obj in changes.iter_mut() {
        obj.bypass_change_detection().clear_changes();
    }

    for mut sets in player_local_changes.iter_mut() {
        sets.bypass_change_detection().0.iter_mut().for_each(|(_, obj)| {
            obj.clear_changes();
        });
    }
}