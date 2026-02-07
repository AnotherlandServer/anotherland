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

use std::sync::Arc;

use bevy::{ecs::{component::Component, error::Result, system::EntityCommands}, state::commands::CommandsStatesExt};
use futures::TryStreamExt;
use realm_api::{ObjectPlacement, RealmApi, Zone};

use crate::{instance::InstanceState, plugins::{LoadContext, LoadableComponent, NonPlayerGameObjectLoader, NonPlayerGameObjectLoaderParams, VirtualComponent}};

#[derive(Component)]
pub struct ZoneLoader;

pub struct ZoneLoaderContext {
    placements: Vec<ObjectPlacement>,
}

pub struct ZoneLoaderParameter {
    pub zone: Arc<Zone>,
}

impl VirtualComponent for ZoneLoader {}

impl LoadableComponent for ZoneLoader {
    type Parameters = ZoneLoaderParameter;
    type ContextData = ZoneLoaderContext;

    async fn load(parameters: Self::Parameters, context: &mut LoadContext<Self::ContextData>) -> Result<Self> {
        let mut query = RealmApi::get()
            .query_object_placements()
            .zone_guid(*parameters.zone.guid())
            .query()
            .await?;

        let mut placements = vec![];
        
        while let Some(placement) = query.try_next().await? {
            placements.push(placement);
        }

        context.set_data(ZoneLoaderContext {
            placements 
        });

        Ok(ZoneLoader)
    }

    fn load_dependencies(&mut self, commands: &mut EntityCommands<'_>, context: &mut LoadContext<Self::ContextData>) -> Result<()> {
        context
            .data_mut()
            .as_mut()
            .unwrap()
            .placements
            .drain(..)
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|placement| {
                let ent = commands
                    .commands()
                    .spawn_empty()
                    .id();

                context.load_cross_dependency::<NonPlayerGameObjectLoader>(
                    ent, 
                    NonPlayerGameObjectLoaderParams::Placement(placement));
            });

        Ok(())
    }

    fn post_load(&mut self, commands: &mut EntityCommands<'_>, _data: Option<Self::ContextData>) -> Result<()> {
        commands
            .commands()
            .set_state(InstanceState::Initializing);

        commands
            .despawn();

        Ok(())
    }
}
