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

use std::{path::PathBuf, str::FromStr, sync::Arc, time::Duration, future::Future};

use bevy::{app::{Last, Main, MainSchedulePlugin, PanicHandlerPlugin, PreStartup, SubApp, TaskPoolPlugin}, diagnostic::FrameCountPlugin, ecs::{component::Component, entity::Entity, message::MessageRegistry, resource::Resource, schedule::{IntoScheduleConfigs, ScheduleLabel}, system::Commands}, prelude::{AppExtStates, AppTypeRegistry,NextState, OnEnter, Query, Res, ResMut}, state::{app::StatesPlugin, state::States}, tasks::futures_lite::StreamExt, time::{TimePlugin, common_conditions::on_timer}};
use core_api::CoreApi;
use derive_builder::Builder;
use log::{debug, trace, error};
use mlua::LuaSerdeExt;
use obj_params::{Class, OaZoneConfig};
use realm_api::{proto::RealmClient, Category, RealmApi, WorldDef, Zone};
use scripting::{LuaRuntime, LuaRuntimeBuilder, ScriptObject, ScriptingPlugin};
use serde_json::Value;
use tokio::{runtime::Handle, task::JoinHandle};
use tokio_util::task::TaskTracker;
use toolkit::types::Uuid;

use crate::{ARGS, error::{WorldError, WorldResult}, instance::InstanceLabel, manager::InstanceManager, plugins::{AbilitiesPlugin, AsyncOperationPlugin, AvatarPlugin, BehaviorPlugin, BuffsPlugin, CashShopPlugin, ChatPlugin, ClientSyncPlugin, CombatPlugin, CombatStylesPlugin, CommandsPlugin, CooldownGroups, DialoguePlugin, FactionsPlugin, InterestsPlugin, InventoryPlugin, LifetimePlugin, LoaderPlugin, MovementPlugin, NavigationPlugin, Navmesh, NetworkPlugin, NpcAiPlugin, PartitioningPlugin, PlayerController, PlayerPlugin, QuestsPlugin, ScriptObjectInfoPlugin, ServerActionPlugin, SocialPlugin, SpecialEventsPlugin, TravelPlugin, WorldSpace}};

#[derive(ScheduleLabel, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct InstanceShutdown;

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
    QuestInit,
    ObjectLoad,
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
    pub core_api: CoreApi,
    pub realm_client: Arc<RealmClient>,
    handle: Handle,
    task_tracker: TaskTracker,

    pub manager: InstanceManager,

    #[builder(setter(strip_option))]
    pub world_def: Arc<WorldDef>,

    #[builder(setter(strip_option))]
    pub zone: Arc<Zone>,

    #[builder(default, setter(skip))]
    pub config: Arc<ZoneConfig>, 

    #[builder(default)]
    pub instance_id: Option<Uuid>,

    #[builder(setter(custom))]
    pub world_controller: Entity,
}

impl ZoneInstance {
    pub fn spawn_task<F: Send + 'static>(&self, task: impl Future<Output = F> + Send + 'static) -> JoinHandle<F>
    {
        self.task_tracker.spawn_on(task, &self.handle)
    }

    pub fn task_tracker(&self) -> TaskTracker {
        self.task_tracker.clone()
    }
}

impl ZoneInstanceBuilder {
    pub async fn instantiate(mut self) -> WorldResult<SubApp> {
        self.world_controller = Some(Entity::PLACEHOLDER);

        let content_path = std::env::var("CONTENT_PATH")
            .ok()
            .and_then(|p| p.parse::<PathBuf>().ok())
            .or(std::env::current_dir().map(|p| p.join("content")).ok())
            .expect("content path inacessible");

        let mut app = SubApp::new();
        let mut instance = self.build()?;

        let world_def = instance.world_def.clone();

        if let Some(config) = RealmApi::get()
            .query_object_templates()
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
        app.init_resource::<MessageRegistry>();
        //app.init_resource::<EventRegistry>();

        app.update_schedule = Some(Main.intern());
        app.add_plugins(TaskPoolPlugin::default());
        app.add_plugins(FrameCountPlugin);
        app.add_plugins(MainSchedulePlugin);
        app.add_plugins(StatesPlugin);
        app.add_plugins(PanicHandlerPlugin);
        app.add_plugins(TimePlugin);

        // Instance setup
        app.init_state::<InstanceState>();
        app.insert_resource(instance);

        // Core plugins
        app.add_plugins((
            AsyncOperationPlugin,
            NetworkPlugin,
            ScriptingPlugin,
            CommandsPlugin,
        ));

        app.insert_resource(
            LuaRuntimeBuilder::default()
                .hot_reload(ARGS.hot_reload)
                .add_require_lookup_directory(content_path.join("lua"))
                .add_require_lookup_directory(content_path.join("lua").join("global").join("scripts"))
                .add_require_lookup_directory(content_path.join("lua").join("maps").join(world_def.name()))
                .build()?
        );

        app.add_plugins(ScriptObjectInfoPlugin);

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
            InventoryPlugin,
        ));

        app.add_plugins((
            SpecialEventsPlugin::new(world_def.name()).await?,
            QuestsPlugin,
            ChatPlugin,
            AbilitiesPlugin,
            CombatPlugin,
            BuffsPlugin,
            NavigationPlugin,
            NpcAiPlugin,
            PartitioningPlugin,
            LifetimePlugin,
        ));

        let navmesh = Navmesh::load(world_def.as_ref()).await?;

        app.insert_resource(WorldSpace::new(navmesh.bounds()));
        app.insert_resource(navmesh);
        app.insert_resource(CooldownGroups::load().await?);

        app.add_systems(PreStartup, spawn_world_controller);
        app.add_systems(OnEnter(InstanceState::Initializing), start_instance);

        app.add_systems(Last,
            check_inactivity_timeout
                .run_if(on_timer(Duration::from_secs(60)))
        );

        app.world_mut().flush();

        Ok(app)
    }
}

#[derive(Component)]
pub struct WorldController;

fn spawn_world_controller(
    mut instance: ResMut<ZoneInstance>,
    mut runtime: ResMut<LuaRuntime>,
    mut commands: Commands,
) {
    let mut controller_scripts = vec![];

    controller_scripts.push(format!("maps.{}.world", instance.world_def.name()));
    controller_scripts.push("core.base_world".to_string());

    for script_name in &controller_scripts {
        match runtime.load_class(script_name) {
            Ok(lua_class) => {
                let obj = ScriptObject::new(&runtime, Some(lua_class)).unwrap();

                let world = runtime.vm().create_table().unwrap();
                world.set("id", *instance.world_def.id()).unwrap();
                world.set("guid", instance.world_def.guid().to_string()).unwrap();
                world.set("name", instance.world_def.name()).unwrap();
                world.set("umap_guid", instance.world_def.umap_guid().to_string()).unwrap();

                let zone = runtime.vm().create_table().unwrap();
                zone.set("id", *instance.zone.id()).unwrap();
                zone.set("guid", instance.zone.guid().to_string()).unwrap();
                zone.set("zone", instance.zone.zone()).unwrap();
                zone.set("is_instance", *instance.zone.is_instance()).unwrap();
                zone.set("realu_zone_type", instance.zone.realu_zone_type()).unwrap();

                let cfg = runtime.vm().create_table().unwrap();
                cfg.set("force_generate_guid_key", instance.config.force_generate_guid_key).unwrap();
                cfg.set("allow_summon_portal", instance.config.allow_summon_portal).unwrap();
                cfg.set("spawn_to_the_last_save_position", instance.config.spawn_to_the_last_save_position).unwrap();
                //cfg.set("instance_type", instance.config.instance_type).unwrap();
                cfg.set("instance_scope", instance.config.instance_scope).unwrap();
                //cfg.set("zone_type", instance.config.zone_type as i32).unwrap();
                let settings = runtime.vm().to_value(&instance.config.json_config).unwrap();
                if settings == runtime.vm().null() {
                    cfg.set("settings", runtime.vm().create_table().unwrap()).unwrap();
                } else {
                    cfg.set("settings", settings).unwrap();
                }

                obj.object().set("world", world).unwrap();
                obj.object().set("zone", zone).unwrap();
                obj.object().set("conf", cfg).unwrap();

                let id = commands.spawn((
                    WorldController, 
                    obj
                )).id();

                instance.world_controller = id;
                break;
            },
            Err(e) => {
                if matches!(e, scripting::ScriptError::FileNotFound(_)) {
                    continue;
                }

                error!("Failed to load script '{script_name}': {e}");
                break;
            },
        }
    }
}

fn start_instance(mut next_state: ResMut<NextState<InstanceState>>) {
    next_state.set(InstanceState::Running);
}

fn check_inactivity_timeout(
    controllers: Query<&PlayerController>,
    instance: Res<ZoneInstance>,
) {
    if controllers.is_empty() {
        debug!("No players in zone, shutting down instance...");

        let label = InstanceLabel::new(*instance.zone.guid(), instance.instance_id);
        let manager = instance.manager.clone();
        instance.spawn_task(async move {
            manager.request_unregister_instance(label).await;
        });
    } else {
        trace!("Instance {}-{:?} passed inactivity check...", instance.zone.guid(), instance.instance_id);
    }
}
