use std::{collections::HashMap, io, ops::{Deref, DerefMut}, convert::{Into, TryFrom}, cell::{RefCell, Ref, RefMut}, sync::Arc};

use atlas::{Uuid, NpcOtherlandParam, StructureParam, PortalParam, StartingPointParam, TriggerParam, SpawnNodeParam, BoundParamClass, ParamClassContainer, ParamError};
use atlas::AvatarId;
use log::{info, error, warn, debug};
use mongodb::Database;
use rand::{thread_rng, Rng};
use tokio::sync::{RwLock, Mutex};

use crate::{util::AnotherlandResult, db::{realm_database, Instance, WorldDef, NpcContent, StructureContent, Content}, world::{NpcAvatar, StructureAvatar, PortalAvatar, StartingPointAvatar, TriggerAvatar, SpawnNodeAvatar}};
use crate::db::{ZoneDef, DatabaseRecord};

use super::{Avatar};

async fn load_instanced_content<'a, T1, T2>(db: Database, instance: &'a Instance) -> AnotherlandResult<T1> 
    where 
        T1: DatabaseRecord<'a, Key = Uuid> + DerefMut<Target = Content>,
        T2: BoundParamClass,
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

pub struct Zone {
    pub worlddef: WorldDef,
    pub zonedef: ZoneDef,
    pub avatars: HashMap<AvatarId, Arc<RwLock<Avatar>>>,
}

impl Zone {
    pub async fn initialize(worlddef: &WorldDef, zonedef: &ZoneDef) -> AnotherlandResult<Zone> {
        let db = realm_database().await;

        let mut zone = Zone {
            avatars: HashMap::new(),
            worlddef: worlddef.clone(),
            zonedef: zonedef.clone(),
        };

        let instances = Instance::load_for_zone(db.clone(), &zone.zonedef.guid).await?;
        for instance in &instances {
            let avatar = match instance.class {
                47 => match load_instanced_content::<NpcContent, NpcOtherlandParam>(db.clone(), instance).await {
                        Ok(content) => NpcAvatar::new(content).into(),
                        Err(e) => {
                            warn!("{:#?}", e); 
                            continue;
                        },
                    },
                55 => match load_instanced_content::<StructureContent, StructureParam>(db.clone(), instance).await {
                    Ok(content) => StructureAvatar::new(content).into(),
                    Err(e) => {
                        warn!("{:#?}", e); 
                        continue;
                    },
                },
                56 => match load_instanced_content::<StructureContent, PortalParam>(db.clone(), instance).await {
                    Ok(content) => PortalAvatar::new(content).into(),
                    Err(e) => {
                        warn!("{:#?}", e); 
                        continue;
                    },
                },
                57 => match load_instanced_content::<StructureContent, StartingPointParam>(db.clone(), instance).await {
                    Ok(content) => StartingPointAvatar::new(content).into(),
                    Err(e) => {
                        warn!("{:#?}", e); 
                        continue;
                    },
                },
                61 => match load_instanced_content::<StructureContent, TriggerParam>(db.clone(), instance).await {
                    Ok(content) => TriggerAvatar::new(content).into(),
                    Err(e) => {
                        warn!("{:#?}", e); 
                        continue;
                    },
                },
                71 => match load_instanced_content::<StructureContent, SpawnNodeParam>(db.clone(), instance).await {
                    Ok(content) => SpawnNodeAvatar::new(content).into(),
                    Err(e) => {
                        warn!("{:#?}", e); 
                        continue;
                    },
                },
                _ => continue, //todo!("loader for content class {}", instance.class),
            };

            zone.spawn_avatar(avatar);
        }

        info!("Loaded {} avatars for zone {}/{}", instances.len(), zone.worlddef.id, zone.zonedef.guid.to_string());       

        Ok(zone)
    }

    pub fn spawn_avatar(&mut self, avatar: Avatar) -> AvatarId {
        // Avatar IDs are prefixed with the avatar type
        let avatar_flag = match avatar {
            Avatar::Player(_) => 0x01,
            Avatar::Npc(_) => 0x02,
            Avatar::Structure(_) => 0x02,
            Avatar::Portal(_) => 0x02,
            Avatar::StartingPoint(_) => 0x02,
            Avatar::Trigger(_) => 0x02,
            Avatar::SpawnNode(_) => 0x02,
        };

        // generate avatar id
        let mut rng = thread_rng();
        let id: AvatarId = loop {
            let id = AvatarId::new((rng.gen_range(0..1<<56) << 0xF) | avatar_flag);
            if !self.avatars.contains_key(&id) {
                break id;
            }
        };

        // add to internal map
        self.avatars.insert(id.clone(), Arc::new(RwLock::new(avatar)));

        id
    }

    pub fn despawn_avatar(&mut self, avatar_id: &AvatarId) {
        self.avatars.remove(avatar_id);
    }

    pub fn get_avatar<'a>(&'a self, avatar_id: &AvatarId) -> Option<Arc<RwLock<Avatar>>> {
        self.avatars.get(avatar_id).map(|r| r.clone())
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, AvatarId, Arc<RwLock<Avatar>>> {
        self.avatars.iter()
    }

    pub fn tick(delta: f32) {
        todo!()
    }

    //fn update_player_avatar_interests(&)
}