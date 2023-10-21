use std::collections::HashMap;
use std::sync::Arc;
use std::{ops::DerefMut, io};

use atlas::{BoundParamClass, ParamError, Uuid, ParamEntity, AvatarId};
use atlas::{NpcOtherlandParam, StructureParam, PortalParam, StartingPointParam, TriggerParam, SpawnNodeParam};
use legion::*;
use legion::storage::IntoComponentSource;
use log::{info, warn};
use mongodb::Database;
use atlas::ParamClassContainer;
use rand::{thread_rng, Rng};
use tokio::sync::RwLock;

use crate::db::ZoneDef;
use crate::util::AnotherlandError;
use crate::{util::AnotherlandResult, db::{Instance, DatabaseRecord, Content, NpcContent, StructureContent}};

use super::AvatarType;

pub struct Zone {
    zonedef: ZoneDef,
    instance: Arc<RwLock<World>>,
    avatar_entity_map: HashMap<AvatarId, Entity>,
}

impl Zone {
    pub fn zonedef(&self) -> &ZoneDef { &self.zonedef }
    pub fn instance(&self) -> &Arc<RwLock<World>> { &self.instance }

    pub async fn spawn_avatar<T>(&mut self, avatar_type: AvatarType, components: T) -> (AvatarId, Entity) 
        where Option<T>: IntoComponentSource
    {
        // Avatar IDs are prefixed with the avatar type
        let avatar_flag = match avatar_type {
            AvatarType::Player => 0x01,
            AvatarType::Npc => 0x02,
            AvatarType::Structure => 0x02,
            AvatarType::Portal => 0x02,
            AvatarType::StartingPoint => 0x02,
            AvatarType::Trigger => 0x02,
            AvatarType::SpawnNode => 0x02,
        };

        // generate avatar id
        let id = {
            let mut rng = thread_rng();
            loop {
                let id = AvatarId::new((rng.gen_range(0..1<<56) << 0xF) | avatar_flag);
                if !self.avatar_entity_map.contains_key(&id) {
                    break id;
                }
            }
        };


        let entity = self.instance.write().await.push(components);
        self.instance.write().await.entry(entity).unwrap().add_component((id.clone(), avatar_type));

        self.avatar_entity_map.insert(id.clone(), entity.clone());
        (id, entity)
    }
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
                let mut class = <ParamClassContainer as TryInto<T2>>::try_into(content.data.as_ref().unwrap().clone())?;
                class.apply(instance_data.to_owned().try_into()?).into()
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
    };

    let instances = Instance::load_for_zone(db.clone(), &zone.zonedef.guid).await?;
    for instance in &instances {
        match instance.class {
            47 => match load_instanced_content::<NpcContent, NpcOtherlandParam>(db.clone(), instance).await {
                    Ok(content) => 
                        zone.spawn_avatar(AvatarType::Npc, <ParamClassContainer as TryInto<NpcOtherlandParam>>::try_into(
                            content.data.as_ref().unwrap().clone()
                        )?.to_entity()).await,
                    Err(e) => {
                        warn!("{:#?}", e); 
                        continue;
                    },
                },
            55 => match load_instanced_content::<StructureContent, StructureParam>(db.clone(), instance).await {
                Ok(content) =>
                    zone.spawn_avatar(AvatarType::Npc, <ParamClassContainer as TryInto<StructureParam>>::try_into(
                        content.data.as_ref().unwrap().clone()
                    )?.to_entity()).await,
                Err(e) => {
                    warn!("{:#?}", e); 
                    continue;
                },
            },
            56 => match load_instanced_content::<StructureContent, PortalParam>(db.clone(), instance).await {
                Ok(content) => 
                    zone.spawn_avatar(AvatarType::Npc, <ParamClassContainer as TryInto<PortalParam>>::try_into(
                        content.data.as_ref().unwrap().clone()
                    )?.to_entity()).await,
                Err(e) => {
                    warn!("{:#?}", e); 
                    continue;
                },
            },
            57 => match load_instanced_content::<StructureContent, StartingPointParam>(db.clone(), instance).await {
                Ok(content) => 
                    zone.spawn_avatar(AvatarType::Npc, <ParamClassContainer as TryInto<StartingPointParam>>::try_into(
                        content.data.as_ref().unwrap().clone()
                    )?.to_entity()).await,
                Err(e) => {
                    warn!("{:#?}", e); 
                    continue;
                },
            },
            61 => match load_instanced_content::<StructureContent, TriggerParam>(db.clone(), instance).await {
                Ok(content) => 
                    zone.spawn_avatar(AvatarType::Npc, <ParamClassContainer as TryInto<TriggerParam>>::try_into(
                        content.data.as_ref().unwrap().clone()
                    )?.to_entity()).await,
                Err(e) => {
                    warn!("{:#?}", e); 
                    continue;
                },
            },
            71 => match load_instanced_content::<StructureContent, SpawnNodeParam>(db.clone(), instance).await {
                Ok(content) => 
                    zone.spawn_avatar(AvatarType::Npc, <ParamClassContainer as TryInto<SpawnNodeParam>>::try_into(
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

    info!("Loaded {} avatars for zone {}-{}", instances.len(), zone.zonedef.guid.to_string(), zone.zonedef.zone);       

    Ok(zone)
}