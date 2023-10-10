use std::io;
use std::{marker::PhantomData, collections::HashMap, ops::Shl};

use atlas::Uuid;
use log::{debug, info};
use rand::{thread_rng, Rng};
use crate::db::DatabaseRecord;
use crate::world::Zone;

use crate::{util::AnotherlandResult, db::{Instance, ZoneDef, WorldDef, realm_database}};

use super::{Avatar::{*, self}, NpcAvatar, StructureAvatar, PortalAvatar, StartingPointAvatar, TriggerAvatar, SpawnNodeAvatar};

pub struct World {
    worlddef: WorldDef,
    zones: HashMap<Uuid, Zone>,
}

impl World {
    pub async fn load_from_id(world_id: u16) -> AnotherlandResult<World> {
        let db = realm_database().await;
        let worlddef = WorldDef::get(db.clone(), &world_id).await?
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "world not found"))?;

        let mut world = World {
            worlddef,
            zones: HashMap::new(),
        };

        let zones = ZoneDef::load_for_world(db.clone(), &world.worlddef.guid).await?;
        for zone in zones {
            world.zones.insert(zone.guid.clone(), Zone::initialize(&world.worlddef, &zone).await?);
        }

        Ok(world)
    }

    pub fn get_zone(&self, guid: &Uuid) -> Option<&Zone> {
        self.zones.get(guid)
    }

    pub fn get_zone_mut(&mut self, guid: &Uuid) -> Option<&mut Zone> {
        self.zones.get_mut(guid)
    }
}