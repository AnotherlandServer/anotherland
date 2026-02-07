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
use realm_api::{Character, ObjectPlacement};
use toolkit::types::Uuid;

use crate::plugins::{LoadContext, LoadableComponent, VirtualComponent};

#[derive(Component)]
pub struct AvatarLoader {
    data: Option<AvatarLoaderData>,
}

pub enum AvatarLoaderData {
    PlayerCharacter(Character),
    Placement(ObjectPlacement),
}

pub enum AvatarLoaderParams {
    PlayerCharacter(Uuid),
    Placement(Uuid),
    PersistentInstance(Uuid),
}

impl VirtualComponent for AvatarLoader {}

impl LoadableComponent for AvatarLoader {
    type Parameters = AvatarLoaderParams;

    async fn load(parameters: Self::Parameters, _context: &mut LoadContext<Self::ContextData>) -> Result<Self> {
        Ok(AvatarLoader {
            data: Some(
                match parameters {
                    AvatarLoaderParams::PlayerCharacter(character_id) => {
                        AvatarLoaderData::PlayerCharacter(
                            Self::load_player_character(character_id).await?
                        )
                    }
                    AvatarLoaderParams::Placement(_) => todo!(),
                    AvatarLoaderParams::PersistentInstance(_) => todo!(),
                }
            ),
        })
    }

    fn load_dependencies(&mut self, commands: &mut EntityCommands<'_>, context: &mut LoadContext<Self::ContextData>) -> Result<()> {
        let mut data = self.data.take();

        let ret = match data.as_mut() {
            Some(AvatarLoaderData::PlayerCharacter(character)) => self.load_player_character_dependencies(commands, context, character),
            None => unreachable!(),
            _ => todo!(),
        };

        self.data = data;
        ret
    }

    fn post_load(&mut self, commands: &mut EntityCommands<'_>, _data: Option<Self::ContextData>) -> Result<()> {
        match self.data.take() {
            Some(AvatarLoaderData::PlayerCharacter(character)) => self.on_load_player_character(commands, character),
            None => unreachable!(),
            _ => todo!(),
        }
    }
}