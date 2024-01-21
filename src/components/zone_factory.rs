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

use std::{collections::{HashMap, HashSet}, sync::Arc};

use atlas::{Uuid, AvatarId, AvatarType};
use log::debug;
use mongodb::Database;
use rand::{thread_rng, Rng};
use tokio::sync::OnceCell;

use crate::{db::{ZoneDef, WorldDef, Instance, MiscContent}, util::{AnotherlandError, AnotherlandResult}, cluster::ActorRef, actors::Zone, NODE};

struct ZoneFactoryData {
    db: Database,
    zone_def: ZoneDef,
    world_def: WorldDef,
    config: OaZoneConfigParam,

    instance_template: OnceCell<Vec<(Instance, AvatarId)>>,
}

#[derive(Clone)]
pub struct ZoneFactory(Arc<ZoneFactoryData>);

impl ZoneFactory {
    pub async fn new(db: Database, world_def: WorldDef, zone_def: ZoneDef) -> AnotherlandResult<ZoneFactory> {
        let config = if zone_def.realu_zone_type.is_empty() {
            OaZoneConfigParam::default()
        } else {
            debug!("Loading zoneconfig: {}", &zone_def.realu_zone_type);

            MiscContent::get_by_name(db.clone(), &zone_def.realu_zone_type)
                .await?
                .map(|mut v| {
                    v.data.take().map(|v| match v {
                        ParamClassContainer::OaZoneConfig(config) => Some(config),
                        _ => None
                    })
                })
                .flatten().flatten()
                .ok_or(AnotherlandError::app_err("zoneconfig not found"))?
        };

        Ok(Self(Arc::new(ZoneFactoryData {
            db,
            zone_def,
            world_def,
            config,
            instance_template: OnceCell::new(),
        })))
    }

    async fn load_content(db: Database, zone_id: &Uuid) -> AnotherlandResult<Vec<(Instance, AvatarId)>> {
        let mut instance_template = Vec::new();
        let mut id_set = HashSet::new();

        let instances = Instance::load_for_zone(db, &zone_id).await?;
        for instance in instances.into_iter() {
            // generate avatar id
            let id = {
                let mut rng = thread_rng();
                loop {
                    let id = AvatarId::new(rng.gen_range(1..1<<56) << 0xF, AvatarType::Npc);
                    if !id_set.contains(&id) {
                        break id;
                    }
                }
            };

            // remember id to avoid collisions
            id_set.insert(id.clone()); 

            instance_template.push((instance, id));
        }

        Ok(instance_template)
    }

    pub async fn spawn_zone(&self) -> ActorRef<Zone> {
        self.0.instance_template.get_or_try_init(|| Self::load_content(self.0.db.clone(), &self.0.zone_def.guid)).await
            .expect("Failed to load in zone content");

        NODE.add_actor(Zone::initialize(self.clone()))
    }

    pub fn db(&self) -> &Database { &self.0.db }
    pub fn zone_def(&self) -> &ZoneDef { &self.0.zone_def }
    pub fn world_def(&self) -> &WorldDef { &self.0.world_def }
    pub fn config(&self) -> &OaZoneConfigParam { &self.0.config }
    pub fn instances(&self) -> &[(Instance, AvatarId)] { self.0.instance_template.get().unwrap().as_slice() }
}