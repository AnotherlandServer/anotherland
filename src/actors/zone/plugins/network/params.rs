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

use atlas::CPktAvatarUpdate;
use bevy_ecs::{event::EventReader, system::Query};
use bitstream_io::{ByteWriter, LittleEndian};
use log::debug;

use crate::actors::{zone::plugins::{ParamsChangedEvent, SubjectiveParamsChangedEvent}, AvatarComponent, InterestList};

use super::PlayerController;

pub fn send_param_updates(
    mut ev: EventReader<ParamsChangedEvent>,
    players: Query<(&AvatarComponent, &InterestList, &PlayerController)>,
) {
    for ParamsChangedEvent(entity, avatar_id, params) in ev.read() {
        // check player interest list to dispatch updates
        for (player_avatar, interests, controller) in players.iter() {
            if interests.contains(*entity) || *avatar_id == player_avatar.id {
                let mut param_buffer = Vec::new();
                let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

                params.write_to_client(&mut writer).expect("failed to serialize params");

                debug!("Send param update for avatar: {}", avatar_id);
                debug!("{:?}", params);

                controller.send_message(CPktAvatarUpdate {
                    full_update: false,
                    avatar_id: Some(*avatar_id),
                    update_source: 0,
                    params: param_buffer.into(),
                    ..Default::default()
                }.into_message());
            }
        }
    }
}

pub fn send_subjective_param_updates(
    mut ev: EventReader<SubjectiveParamsChangedEvent>,
    players: Query<(&AvatarComponent, &InterestList, &PlayerController)>,
) {
    for SubjectiveParamsChangedEvent { entity, avatar, player, params } in ev.read() {
        // check player interest list to dispatch updates
        if let Ok((player_avatar, interests, controller)) = players.get(*player) {
            if interests.contains(*entity) || *avatar == player_avatar.id {
                let mut param_buffer = Vec::new();
                let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

                params.write_to_client(&mut writer).expect("failed to serialize params");

                debug!("Send subjective param update for avatar: {} for player {}", avatar, player_avatar.name);
                debug!("{:?}", params);

                controller.send_message(CPktAvatarUpdate {
                    full_update: false,
                    avatar_id: Some(*avatar),
                    update_source: 0,
                    params: param_buffer.into(),
                    ..Default::default()
                }.into_message());
            }
        }
    }
}