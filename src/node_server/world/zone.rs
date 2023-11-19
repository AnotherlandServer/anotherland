// Copyright (C) 2023 AnotherlandServer
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

use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use std::{ops::DerefMut, io};

use atlas::{BoundParamClass, ParamError, Uuid, ParamEntity, AvatarId, NonClientBaseComponent, StartingPointComponent, NonClientBase, ParamClass};
use atlas::{NpcOtherlandParam, StructureParam, PortalParam, StartingPointParam, TriggerParam, SpawnNodeParam};
use glam::Vec3;
use legion::*;
use legion::storage::{IntoComponentSource, PackOptions};
use log::{info, warn, debug, error};
use mongodb::Database;
use atlas::ParamClassContainer;
use rand::{thread_rng, Rng};
use rsa::traits::PublicKeyParts;
use tokio::sync::RwLock;

use crate::db::ZoneDef;
use crate::util::AnotherlandError;
use crate::{util::AnotherlandResult, db::{Instance, DatabaseRecord, Content, NpcContent, StructureContent}};

use super::{AvatarType, AvatarComponent};

pub struct Zone {
    zonedef: ZoneDef,
    instance: Arc<RwLock<World>>,
    avatar_entity_map: HashMap<AvatarId, Entity>,
    start_pos: Vec3,
    start_rot: Vec3,
}

impl Zone {
    pub fn zonedef(&self) -> &ZoneDef { &self.zonedef }
    pub fn instance(&self) -> &Arc<RwLock<World>> { &self.instance }

    pub async fn remove_avatar(&mut self, avatar_id: &AvatarId) {
        if let Some(entity) = self.avatar_entity_map.remove(avatar_id) {
            self.instance.write().await.remove(entity);
        }
    }

    pub async fn spawn_avatar<T>(&mut self, avatar_type: AvatarType, id: Option<AvatarId>, name: &str, components: T) -> (AvatarId, Entity) 
        where Option<T>: IntoComponentSource
    {
        // Avatar IDs are prefixed with the avatar type
        let avatar_flag = match avatar_type {
            AvatarType::Player => 0x01,
            AvatarType::NpcOtherland => 0x02,
            AvatarType::Structure => 0x03,
            AvatarType::Portal => 0x03,
            AvatarType::StartingPoint => 0x03,
            AvatarType::Trigger => 0x03,
            AvatarType::SpawnNode => 0x03,
        };

        // generate avatar id
        let id = id.unwrap_or({
            let mut rng = thread_rng();
            loop {
                let id = AvatarId::new((rng.gen_range(1..1<<56) << 0xF) | avatar_flag);
                if !self.avatar_entity_map.contains_key(&id) {
                    break id;
                }
            }
        });

        //debug!("Generate id {:016x}", id.as_u64());

        let entity = self.instance.write().await.push(components);
        self.instance.write().await.entry(entity).unwrap().add_component(avatar_type);
        self.instance.write().await.entry(entity).unwrap().add_component(AvatarComponent {
            id: id.clone(),
            name: name.to_owned(),
            vel: Vec3::default(),
        });

        self.avatar_entity_map.insert(id.clone(), entity.clone());
        (id, entity)
    }

    pub fn start_pos(&self) -> Vec3 { self.start_pos.clone() }
    pub fn start_rot(&self) -> Vec3 { self.start_rot.clone() }
}

async fn load_instanced_content<'a, T1, T2>(db: Database, instance: &'a Instance) -> AnotherlandResult<T1> 
    where 
        T1: DatabaseRecord<'a, Key = Uuid> + DerefMut<Target = Content>,
        T2: BoundParamClass + ParamEntity,
        T2: TryFrom<ParamClassContainer, Error = ParamError>
{
    if let Some(mut content) = T1::get(db.clone(), &instance.content_guid).await? {
        if let Some(instance_data) = &instance.data {
            if content.data.is_some() {
                if let Some(class) = content.data.as_mut() {
                    class.as_anyclass_mut().apply(instance_data.clone().to_anyclass());
                }
                
                //let mut class = <ParamClassContainer as TryInto<&mut T2>>::try_into(content.data.as_ref().unwrap())?;
                //class.apply(instance_data.to_owned().try_into()?).into()
            }
        }

        Ok(content.into())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound, 
            format!("{} {} not found in content db", std::any::type_name::<T1>(), instance.content_guid.to_string())
        ).into())
    }
}

pub async fn load_zone_from_definition(db: Database, zonedef: ZoneDef) -> AnotherlandResult<Zone> {
    let mut zone = Zone {
        zonedef,
        instance: Arc::new(RwLock::new(World::new(WorldOptions::default()))),
        avatar_entity_map: HashMap::new(),
        start_pos: Vec3::default(),
        start_rot: Vec3::default(),
    };

    let instances = Instance::load_for_zone(db.clone(), &zone.zonedef.guid).await?;
    for instance in &instances {
        match instance.class {
            47 => match load_instanced_content::<NpcContent, NpcOtherlandParam>(db.clone(), instance).await {
                    Ok(content) => 
                        zone.spawn_avatar(AvatarType::NpcOtherland, None, &content.name,  <ParamClassContainer as TryInto<NpcOtherlandParam>>::try_into(
                            content.data.as_ref().unwrap().clone()
                        )?.to_entity()).await,
                    Err(e) => {
                        warn!("{:#?}", e); 
                        continue;
                    },
                },
            55 => match load_instanced_content::<StructureContent, StructureParam>(db.clone(), instance).await {
                Ok(content) =>
                    zone.spawn_avatar(AvatarType::Structure, None, &content.name, <ParamClassContainer as TryInto<StructureParam>>::try_into(
                        content.data.as_ref().unwrap().clone()
                    )?.to_entity()).await,
                Err(e) => {
                    warn!("{:#?}", e); 
                    continue;
                },
            },
            56 => match load_instanced_content::<StructureContent, PortalParam>(db.clone(), instance).await {
                Ok(content) => 
                    zone.spawn_avatar(AvatarType::Portal, None, &content.name, <ParamClassContainer as TryInto<PortalParam>>::try_into(
                        content.data.as_ref().unwrap().clone()
                    )?.to_entity()).await,
                Err(e) => {
                    warn!("{:#?}", e); 
                    continue;
                },
            },
            57 => match load_instanced_content::<StructureContent, StartingPointParam>(db.clone(), instance).await {
                Ok(content) => 
                    zone.spawn_avatar(AvatarType::StartingPoint, None, &content.name, <ParamClassContainer as TryInto<StartingPointParam>>::try_into(
                        content.data.as_ref().unwrap().clone()
                    )?.to_entity()).await,
                Err(e) => {
                    warn!("{:#?}", e); 
                    continue;
                },
            },
            61 => match load_instanced_content::<StructureContent, TriggerParam>(db.clone(), instance).await {
                Ok(content) => 
                    zone.spawn_avatar(AvatarType::Trigger, None, &content.name, <ParamClassContainer as TryInto<TriggerParam>>::try_into(
                        content.data.as_ref().unwrap().clone()
                    )?.to_entity()).await,
                Err(e) => {
                    warn!("{:#?}", e); 
                    continue;
                },
            },
            71 => match load_instanced_content::<StructureContent, SpawnNodeParam>(db.clone(), instance).await {
                Ok(content) => 
                    zone.spawn_avatar(AvatarType::SpawnNode, None, &content.name, <ParamClassContainer as TryInto<SpawnNodeParam>>::try_into(
                        content.data.as_ref().unwrap().clone()
                    )?.to_entity()).await,
                Err(e) => {
                    warn!("{:#?}", e); 
                    continue;
                },
            },
            _ => continue, //todo!("loader for content class {}", instance.class),
        };
    }

    /*zone.instance.write().await.pack(PackOptions {
        ..Default::default()
    });*/

    {
        let mut query = <(&NonClientBaseComponent, &StartingPointComponent)>::query();
        let world = zone.instance.read().await;

        let starting_points: Vec<_> = query.iter(world.deref()).collect();
        match starting_points.len() {
            0 => warn!("No starting point found for zone {}-{}", zone.zonedef.guid.to_string(), zone.zonedef.zone),
            1 => {
                zone.start_pos = starting_points[0].0.pos().unwrap().clone();
                zone.start_rot = starting_points[0].0.rot().unwrap().clone();
            },
            _ => error!("Multiple starting points for zone {}-{}", zone.zonedef.guid.to_string(), zone.zonedef.zone),
        }
    }

    info!("Loaded {} avatars for zone {}-{}", instances.len(), zone.zonedef.guid.to_string(), zone.zonedef.zone);       

    Ok(zone)
}