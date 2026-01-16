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

use bevy::ecs::{entity::Entity, system::{Commands, In}};
use toolkit::NativeParam;

use crate::plugins::{AcceptQuest, FailQuest, ReturnQuest};


pub fn command_accept_quest(
    In((ent, args)): In<(Entity, Vec<NativeParam>)>,
    mut commands: Commands,
) {
    let mut args = args.into_iter();

    if let Some(NativeParam::Int(quest_id)) = args.next() {
        commands.write_message(AcceptQuest { 
            player: ent,
            quest_id,
        });
    }
}

pub fn command_complete_quest(
    In((_ent, args)): In<(Entity, Vec<NativeParam>)>,
) {
    let mut args = args.into_iter();

    if let Some(NativeParam::Int(_quest_id)) = args.next() {
        todo!("Implement command_complete_quest");
    }
}

pub fn command_finish_quest(
    In((ent, args)): In<(Entity, Vec<NativeParam>)>,
    mut commands: Commands,
) {
    let mut args = args.into_iter();

    if let Some(NativeParam::Int(quest_id)) = args.next() {
        commands.write_message(ReturnQuest { 
            player: ent,
            quest_id,
        });
    }
}

pub fn command_fail_quest(
    In((ent, args)): In<(Entity, Vec<NativeParam>)>,
    mut commands: Commands,
) {
    let mut args = args.into_iter();

    if let Some(NativeParam::Int(quest_id)) = args.next() {
        commands.write_message(FailQuest { 
            player: ent,
            quest_id,
        });
    }
}
