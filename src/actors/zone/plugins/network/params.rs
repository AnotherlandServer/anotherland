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

use atlas::{CPktAvatarUpdate, ParamBox};
use bevy_ecs::{component::Component, entity::Entity, query::{Added, Changed}, system::{Commands, Query}};
use bitstream_io::{ByteWriter, LittleEndian};
use log::debug;

use crate::actors::{AvatarComponent, InterestList};

use super::PlayerController;

#[derive(Component)]
pub struct PreviousParamBox(ParamBox);

pub fn prepare_param_updates(
    spawned: Query<(Entity, &ParamBox), Added<ParamBox>>,
    mut cmds: Commands,
) {
    for (ent, params) in spawned.iter() {
        cmds.entity(ent)
            .insert(PreviousParamBox(params.clone()));
    }
}

pub fn send_param_updates(
    mut params: Query<(&AvatarComponent, &ParamBox, &mut PreviousParamBox), Changed<ParamBox>>,
    players: Query<(&AvatarComponent, &InterestList, &PlayerController)>,
) {
    for (avatar, params, mut prev_params) in params.iter_mut() {
        let diff = params.diff(&prev_params.0);
        if diff.is_empty() {
            continue;
        }

        // store params for future comparison
        params.clone_into(&mut prev_params.0);

        // check player interest list to dispatch updates
        for (player_avatar, interests, controller) in players.iter() {
            if interests.contains(avatar.id) || avatar.id == player_avatar.id {
                let mut param_buffer = Vec::new();
                let mut writer = ByteWriter::endian(&mut param_buffer, LittleEndian);

                diff.write_to_client(&mut writer).expect("failed to serialize params");

                debug!("Send param update for avatar: {}", avatar.id);
                debug!("{:?}", diff);

                controller.send_message(CPktAvatarUpdate {
                    full_update: false,
                    avatar_id: Some(avatar.id.as_u64()),
                    update_source: 0,
                    params: param_buffer.into(),
                    ..Default::default()
                }.into_message());
            }
        }
    }
}