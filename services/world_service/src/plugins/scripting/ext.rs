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

use bevy::ecs::{entity::Entity, message::{Message, MessageReader, Messages}, system::{Commands, EntityCommands}};
use log::debug;

#[derive(Message)]
pub struct DespawnEntity(pub Entity);

pub trait ScriptingEntityCommandsExt {
    fn deferred_despawn(&mut self) -> &mut Self;
}

impl ScriptingEntityCommandsExt for EntityCommands<'_> {
    fn deferred_despawn(&mut self) -> &mut Self {
        let id = self.id();

        self
            .commands()
            .write_message(DespawnEntity(id));

        self
    }
}

pub fn handle_despawn_entity(mut messages: MessageReader<DespawnEntity>, mut commands: Commands) {
    for &DespawnEntity(entity) in messages.read() {
        debug!("Despawning entity {}", entity);
        commands.entity(entity).despawn();
    }
}