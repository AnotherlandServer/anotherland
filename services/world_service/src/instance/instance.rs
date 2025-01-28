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

use std::{path::PathBuf, str::FromStr, sync::Arc};

use bevy::{app::{First, Main, MainSchedulePlugin, PanicHandlerPlugin, ScheduleRunnerPlugin, SubApp}, core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin}, ecs::{event::{event_update_condition, event_update_system, EventRegistry, EventUpdates}, intern::Interned, schedule::ScheduleLabel}, log::LogPlugin, prelude::{AppExtStates, AppTypeRegistry, HierarchyPlugin, IntoSystemConfigs, NextState, OnEnter, ResMut, Resource, States}, state::app::StatesPlugin, tasks::futures_lite::StreamExt, time::TimePlugin, MinimalPlugins};
use core_api::CoreApi;
use derive_builder::Builder;
use log::debug;
use obj_params::{Class, OaZoneConfig};
use realm_api::{proto::RealmClient, Category, RealmApi, WorldDef, Zone};
use scripting::{LuaRuntimeBuilder, ScriptingPlugin};
use serde_json::Value;
use tokio::runtime::Handle;
use toolkit::types::Uuid;

use crate::{error::{WorldError, WorldResult}, object_cache::ObjectCache, plugins::{AbilitiesPlugin, AvatarPlugin, BehaviorPlugin, CashShopPlugin, ChatPlugin, ClientSyncPlugin, CombatPlugin, CombatStylesPlugin, CommandsPlugin, DialoguePlugin, FactionsPlugin, InterestsPlugin, InventoryPlugin, LoaderPlugin, MovementPlugin, NetworkPlugin, PlayerPlugin, QuestsPlugin, ScriptObjectInfoPlugin, ServerActionPlugin, SocialPlugin, SpecialEventsPlugin, TravelPlugin}, ARGS};

#[derive(Default)]
pub enum ZoneType {
    #[default]
    Generic,
    Dungeon,
    Emergency,
    Minigame,
    MypadRoom,
    MypadFoyer,
}

impl FromStr for ZoneType {
    type Err = WorldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "generic" => Ok(Self::Generic),
            "dungeon" => Ok(Self::Dungeon),
            "emergency" => Ok(Self::Emergency),
            "minigame" => Ok(Self::Minigame),
            "mypadroom" => Ok(Self::MypadRoom),
            "mypadfoyer" => Ok(Self::MypadFoyer),
            _ => Err(WorldError::UnknownZoneType(s.to_owned())),
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
    type Error = WorldError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Persistent),
            2 => Ok(Self::Instanced),
            _ => Err(WorldError::UnknownInstanceType(value)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum InstanceState {
    #[default]
    Loading,
    Initializing,
    Running,
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
    pub json_config: serde_json::Value,
}

#[derive(Builder, Resource)]
#[builder(pattern = "owned", build_fn(private, error = "WorldError"))]
pub struct ZoneInstance {
    pub realm_api: RealmApi,
    pub core_api: CoreApi,
    pub realm_client: Arc<RealmClient>,
    pub handle: Handle,
    pub object_cache: ObjectCache,

    #[builder(setter(strip_option))]
    pub world_def: Arc<WorldDef>,

    #[builder(setter(strip_option))]
    pub zone: Arc<Zone>,

    #[builder(default, setter(skip))]
    pub config: Arc<ZoneConfig>, 

    #[builder(default)]
    pub instance_id: Option<Uuid>,
}

impl ZoneInstanceBuilder {
    pub async fn instantiate(self) -> WorldResult<SubApp> {
        let content_path = std::env::var("CONTENT_PATH")
            .ok()
            .and_then(|p| p.parse::<PathBuf>().ok())
            .or(std::env::current_dir().map(|p| p.join("content")).ok())
            .expect("content path inacessible");

        let mut app = SubApp::new();
        let mut instance = self.build()?;

        let realm_api = instance.realm_api.clone();
        let world_def = instance.world_def.clone();
        let object_cache = instance.object_cache.clone();

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
                json_config: config.data
                    .get::<_, Value>(OaZoneConfig::JsonConfig)?
                    .clone()
            });
        }

        // Low level setup
        app.init_resource::<AppTypeRegistry>();
        app.init_resource::<EventRegistry>();

        app.update_schedule = Some(Main.intern());
        app.add_plugins(TaskPoolPlugin::default());
        app.add_plugins(TypeRegistrationPlugin);
        app.add_plugins(FrameCountPlugin);
        app.add_plugins(TimePlugin);
        app.add_plugins(MainSchedulePlugin);
        app.add_plugins(StatesPlugin);
        app.add_plugins(LogPlugin::default());
        app.add_plugins(PanicHandlerPlugin);
        app.add_plugins(HierarchyPlugin);

        app.add_systems(
            First,
            event_update_system
                .in_set(EventUpdates)
                .run_if(event_update_condition),
        );

        // Instance setup
        app.init_state::<InstanceState>();
        app.insert_resource(instance);
        app.add_systems(OnEnter(InstanceState::Initializing), start_instance);


        // Core plugins
        app.add_plugins((
            NetworkPlugin,
            ScriptingPlugin,
            ScriptObjectInfoPlugin,
            CommandsPlugin,
        ));

        app.insert_resource(
            LuaRuntimeBuilder::default()
                .hot_reload(ARGS.hot_reload)
                .add_require_lookup_directory(content_path.join("lua"))
                .add_require_lookup_directory(content_path.join("lua/scripts"))
                .add_require_lookup_directory(content_path.join("lua/scripts").join(world_def.name()))
                .build()?
        );

        // Game logic plugins
        app.add_plugins((
                AvatarPlugin,
                BehaviorPlugin,
                InterestsPlugin,
                LoaderPlugin,
                MovementPlugin,
                PlayerPlugin,
                ServerActionPlugin,
                SocialPlugin,
                CashShopPlugin,
                ClientSyncPlugin,
                TravelPlugin,
                FactionsPlugin,
                DialoguePlugin,
                CombatStylesPlugin,
                InventoryPlugin {
                    content_path: content_path.clone()
                }
            ));

            app.add_plugins((
                SpecialEventsPlugin::new(object_cache.clone(), realm_api.clone(), world_def.name()).await?,
                QuestsPlugin,
                ChatPlugin,
                AbilitiesPlugin,
                CombatPlugin,
            ));

        Ok(app)
    }
}

fn start_instance(mut next_state: ResMut<NextState<InstanceState>>) {
    next_state.set(InstanceState::Running);
}
