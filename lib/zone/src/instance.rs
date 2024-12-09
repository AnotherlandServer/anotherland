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

use std::{str::FromStr, sync::Arc};

use bevy::{app::SubApp, prelude::Resource, tasks::futures_lite::StreamExt, MinimalPlugins};
use derive_builder::Builder;
use obj_params::{Class, OaZoneConfig};
use realm_api::{Category, RealmApi, Zone};
use toolkit::types::Uuid;

use crate::{ZoneError, ZoneResult};

#[derive(Default)]
pub enum ZoneType {
    #[default]
    Generic,
    Emergency,
    Minigame,
    MypadRoom,
    MypadFoyer,
}

impl FromStr for ZoneType {
    type Err = ZoneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "generic" => Ok(Self::Generic),
            "emergency" => Ok(Self::Emergency),
            "minigame" => Ok(Self::Minigame),
            "mypadroom" => Ok(Self::MypadRoom),
            "mypadfoyer" => Ok(Self::MypadFoyer),
            _ => Err(ZoneError::UnknownZoneType(s.to_owned())),
        }
    }
}

#[derive(Default)]
pub enum InstanceType {
    #[default]
    Persistent,
    Instanced,
}

impl TryFrom<i32> for InstanceType {
    type Error = ZoneError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Persistent),
            2 => Ok(Self::Instanced),
            _ => Err(ZoneError::UnknownInstanceType(value)),
        }
    }
}

#[derive(Builder, Default)]
#[builder(pattern = "owned", default)]
pub struct ZoneConfig {
    pub force_generate_guid_key: bool,
    pub allow_summon_portal: bool,
    pub spawn_to_the_last_save_position: bool,
    pub instance_type: InstanceType,
    pub instance_scope: i32,
    pub zone_type: ZoneType,
}

#[derive(Builder, Resource)]
#[builder(pattern = "owned", build_fn(private, error = "ZoneError"))]
pub struct ZoneInstance {
    realm_api: RealmApi,

    #[builder(setter(strip_option), field(ty = "Option<Zone>", build = "Arc::new(self.zone.ok_or(derive_builder::UninitializedFieldError::new(\"zone\"))?)"))]
    pub zone: Arc<Zone>,

    #[builder(default, setter(skip))]
    pub config: Arc<ZoneConfig>,

    #[builder(default)]
    pub instance_id: Uuid,
}

impl ZoneInstanceBuilder {
    pub async fn instantiate(self) -> ZoneResult<SubApp> {
        let mut app = SubApp::new();
        let mut instance = self.build()?;

        if let Some(config) = instance.realm_api.query_object_templates()
            .category(Category::Misc)
            .class(Class::OaZoneConfig)
            .name(instance.zone.realu_zone_type().to_owned())
            .query().await?
            .try_next().await?
        {
            instance.config = Arc::new(ZoneConfig { 
                force_generate_guid_key: *config.data
                    .get(OaZoneConfig::ForceGenerateGuidKey)?, 
                allow_summon_portal: *config.data
                    .get(OaZoneConfig::AllowSummonPortal)?, 
                spawn_to_the_last_save_position: *config.data
                    .get(OaZoneConfig::SpawnToTheLastSavePosition)?,  
                instance_type: (*config.data
                    .get::<_, i32>(OaZoneConfig::InstanceType)?)
                    .try_into()?,  
                instance_scope: *config.data
                    .get(OaZoneConfig::InstanceScope)?, 
                zone_type: config.data
                    .get::<_, String>(OaZoneConfig::ZoneType)?
                    .parse()?, 
            });
        }

        if instance.config.force_generate_guid_key {
            instance.instance_id = Uuid::new();
        }

        app
            .insert_resource(instance);

        Ok(app)
    }
}