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

use bevy::{app::{Last, Plugin}, prelude::{Changed, DetectChangesMut, Entity, Query}};
use bitstream_io::{ByteWriter, LittleEndian};
use log::debug;
use obj_params::{GameObjectData, ParamWriter};
use protocol::CPktAvatarUpdate;

use super::{AvatarInfo, Interests, PlayerController};

pub struct ClientSyncPlugin;

impl Plugin for ClientSyncPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Last, sync_avatar_data);
    }
}

fn sync_avatar_data(
    mut changes: Query<(Entity, &AvatarInfo, &mut GameObjectData), Changed<GameObjectData>>,
    players: Query<(&AvatarInfo, &Interests, &PlayerController)>,
) {
    for (entity, obj_avatar, mut obj) in changes.iter_mut() {
        let changed_params = obj.changes();
        obj.bypass_change_detection().clear_changes(); // Clear changes for next gametick

        for (player_avatar, interests, controller) in players.iter() {
            if interests.contains(&entity) || obj_avatar.id == player_avatar.id {
                let mut param_buffer = Vec::new();
                let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

                if obj_avatar.id == player_avatar.id {
                    changed_params.write_to_privileged_client(&mut writer).expect("failed to serialize params")
                } else {
                    changed_params.write_to_client(&mut writer).expect("failed to serialize params")
                }

                debug!("Send param update for avatar: {}", obj_avatar.id);
                debug!("{:?}", changed_params);

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