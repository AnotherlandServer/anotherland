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

use bevy::ecs::{component::Component, error::Result, system::EntityCommands};
use realm_api::{Character, ObjectPlacement, RealmApi};
use toolkit::types::Uuid;

use crate::plugins::{LoadContext, LoadableComponent, VirtualComponent};

#[derive(Component)]
pub struct AvatarLoader {
    pub realm_api: RealmApi,
    data: Option<AvatarLoaderData>,
}

pub enum AvatarLoaderData {
    PlayerCharacter(Character),
    Placement(ObjectPlacement),
}

pub struct AvatarLoaderParams {
    pub id: AvatarStorageId,
    pub realm_api: RealmApi,
}

pub enum AvatarStorageId {
    PlayerCharacter(Uuid),
    Placement(Uuid),
    PersistentInstance(Uuid),
}

impl VirtualComponent for AvatarLoader {}

impl LoadableComponent for AvatarLoader {
    type Parameters = AvatarLoaderParams;

    async fn load(parameters: Self::Parameters) -> Result<Self> {
        Ok(AvatarLoader {
            realm_api: parameters.realm_api.clone(),
            data: Some(
                match parameters.id {
                    AvatarStorageId::PlayerCharacter(character_id) => {
                        AvatarLoaderData::PlayerCharacter(
                            Self::load_player_character(parameters.realm_api, character_id).await?
                        )
                    }
                    AvatarStorageId::Placement(_) => todo!(),
                    AvatarStorageId::PersistentInstance(_) => todo!(),
                }
            ),
        })
    }

    fn on_load(&mut self, commands: &mut EntityCommands<'_>, context: &mut LoadContext) -> Result<()> {
        match self.data.take().unwrap() {
            AvatarLoaderData::PlayerCharacter(character) => self.on_load_player_character(commands, context, character),
            _ => todo!(),
        }
    }
}