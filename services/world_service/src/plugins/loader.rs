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

use std::{collections::HashMap, sync::Arc};

use bevy::{app::{First, Plugin, Startup, Update}, prelude::{in_state, Added, Commands, Component, Entity, IntoSystemConfigs, NextState, NonSend, NonSendMut, Query, Res, ResMut}};
use futures_util::TryStreamExt;
use log::{debug, info};
use obj_params::{GameObjectData, tag_gameobject_entity};
use realm_api::{ObjectPlacement, ObjectTemplate, RealmApi};
use tokio::sync::mpsc::{Receiver, Sender};
use toolkit::types::Uuid;

use crate::instance::{InstanceState, ZoneInstance};

use super::Shared;

struct Content(Option<(ObjectPlacement, Arc<ObjectTemplate>)>);

#[derive(Component)]
pub struct ContentInfo {
    pub placement_id: Uuid,
    pub template_id: Uuid,
}

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let (content_sender, content_receiver) = tokio::sync::mpsc::channel::<Content>(100);

        app.insert_resource(Shared(content_receiver));

        app.add_systems(First, (
                ingest_content.run_if(in_state(InstanceState::Initializing)),
                tag_gameobjects
            ).chain());

        let instance = app.world().get_resource::<ZoneInstance>().unwrap();
        let realm_api = instance.realm_api.clone();
        let zone = instance.zone.clone();

        instance.handle.spawn(async move {
            let mut template_cache: HashMap<Uuid, Arc<ObjectTemplate>> = HashMap::new();
            
            // Query
            let mut query = realm_api.query_object_placements()
                .zone_guid(*zone.guid())
                .query().await.unwrap();
    
            
            while let Some(placement) = query.try_next().await.unwrap() {
                let template = if let Some(template) = template_cache.get(&placement.content_guid) {
                    template.clone()
                } else if let Some(template) = realm_api.get_object_template(placement.content_guid).await.unwrap() {
                    let template = Arc::new(template);
                    template_cache.insert(placement.content_guid, template.clone());
                    template
                } else {
                    continue;
                };

                debug!("Loaded placement: {:?}", placement.id);

                if content_sender.send(Content(Some((placement, template)))).await.is_err() {
                    return;
                }
            }
    
            info!("Instance {} load completed.", zone.guid());
            let _ = content_sender.send(Content(None)).await;
        });
    }
}

fn ingest_content(
    mut receiver: ResMut<Shared<Receiver<Content>>>,
    mut next_state: ResMut<NextState<InstanceState>>,
    mut commands: Commands,
) {
    while let Ok(Content(content)) = receiver.try_recv() {
        if let Some((mut placement, template)) = content {
            placement.data.merge(template.data.clone());

            commands.spawn((
                ContentInfo {
                    placement_id: placement.id,
                    template_id: template.id,
                },
                placement.data
            ));
        } else {
            debug!("Done receiving");
            next_state.set(InstanceState::Running);
        }
    }
}

fn tag_gameobjects(
    added: Query<(Entity, &GameObjectData), Added<GameObjectData>>,
    mut commands: Commands,
) {
    for (ent, obj) in added.iter() {
        tag_gameobject_entity(obj, &mut commands.entity(ent));
    }
}