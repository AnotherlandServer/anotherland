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

use std::{ops::Deref, sync::Arc};

use bevy::{app::{First, Last, Plugin, Update}, ecs::{component::Component, event::EventWriter, query::Without, system::{Resource, SystemId}, world::World}, hierarchy::DespawnRecursiveExt, math::{Quat, Vec3}, prelude::{in_state, Added, Changed, Commands, Entity, In, IntoSystemConfigs, Or, Query, Res, ResMut, With}};
use bitstream_io::{ByteWriter, LittleEndian};
use futures::{future::join_all, TryStreamExt};
use log::{debug, error, trace, warn};
use mlua::{FromLua, Function, IntoLua, Lua, Table, UserData};
use obj_params::{tags::{PlayerTag, PortalTag, SpawnNodeTag, StartingPointTag}, AttributeInfo, Class, EdnaAbility, GameObjectData, GenericParamSet, NonClientBase, ParamFlag, ParamSet, ParamWriter, Player, Portal, Value};
use protocol::{oaAbilityBarReferences, oaAbilityDataPlayer, oaAbilityDataPlayerArray, oaPktS2XConnectionState, oaPlayerClassData, AbilityBarReference, CPktAvatarUpdate, CPktBlob, MoveManagerInit, OaPktS2xconnectionStateState, Physics, PhysicsState};
use realm_api::{AbilitySlot, Character, EquipmentResult, RealmApiResult, State};
use scripting::{LuaExt, LuaRuntime, LuaTableExt, ScriptCommandsExt, ScriptObject, ScriptResult};
use tokio::sync::mpsc::{self, Receiver, Sender};
use toolkit::{types::Uuid, NativeParam, OtherlandQuatExt};
use anyhow::anyhow;

use crate::{error::WorldResult, instance::{InstanceState, ZoneInstance}, object_cache::CacheEntry, plugins::{Active, ForeignResource}, proto::TravelMode, OBJECT_CACHE};

use super::{clear_obj_changes, init_gameobjects, load_class_script, AvatarInfo, BehaviorExt, CombatStyle, CommandExtPriv, ConnectionState, ContentInfo, Cooldowns, CurrentState, FutureCommands, HealthUpdateEvent, InitialInventoryTransfer, Movement, NetworkExtPriv, ParamValue, PlayerController, QuestLog, ServerAction, StringBehavior};

#[derive(Debug)]
#[allow(unused)]
pub struct Skill {
    pub id: Uuid,
    pub ability: Arc<CacheEntry>,
    pub group: String,
    pub state: State,
    pub stance: i32,
}

#[derive(Clone, Debug)]
pub struct SkillbookEntry(Arc<Skill>);

impl UserData for SkillbookEntry {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("Get", |lua, this, name: String| {
            let val = this.ability.data.get_named::<obj_params::Value>(&name)
                .map_err(mlua::Error::external)?;
        
            ParamValue::new(val.clone())
                .into_lua(lua)
       });
    }
}

impl FromLua for SkillbookEntry {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        let usr = value.as_userdata().ok_or(mlua::Error::runtime("object expected"))?;
        Ok(usr.borrow::<SkillbookEntry>()?.clone())
    }
}

impl Deref for SkillbookEntry {
    type Target = Skill;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SkillbookEntry {
    pub fn construct_lua_table(&self, runtime: &mut LuaRuntime) -> WorldResult<Table> {
        let base = load_class_script(runtime, 
            self.0.ability.class, 
            self.0.ability.data.get::<_, String>(EdnaAbility::LuaScript).ok().map(|s| s.as_str()))?;

        let metatable = runtime.vm().create_table()?;
        metatable.set("__index", base)?;

        let table = runtime.vm().create_table()?;
        table.set_metatable(Some(metatable));
        table.set("__skill", self.clone())?;

        Ok(table)
    }
}

fn insert_skillbook_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let skillbook_api = lua.create_table().unwrap();
    runtime.register_native("skillbook", skillbook_api.clone()).unwrap();

    skillbook_api.set("GetSkill", lua.create_bevy_function(world, 
        |
            In((player, skill_id)): In<(Table, String)>,
            query: Query<&Skillbook>,
            mut runtime: ResMut<LuaRuntime>,
        | -> WorldResult<Option<Table>> {
            let skillbook = query.get(player.entity()?)
                .map_err(|_| anyhow!("player not found"))?;

            let skill_id = skill_id.parse::<Uuid>()?;

            skillbook.0.iter()
                .find(|s| s.id == skill_id)
                .map(|s| s.construct_lua_table(&mut runtime))
                .transpose()
        })?)?;

    Ok(())
}

fn insert_player_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let player_api = lua.create_table().unwrap();
    runtime.register_native("player", player_api.clone()).unwrap();

    player_api.set("Spawn", lua.create_bevy_function(world, 
        |
            In(player): In<Table>,
            mut query: Query<(Entity, &AvatarInfo, &Movement, &mut PlayerController, &mut CurrentState), Changed<CurrentState>>,
            mut commands: Commands
        | -> WorldResult<()> {
            if let Ok((ent, info, movement, controller, mut state)) = query.get_mut(player.entity()?) {
                debug!("Spawning player: {}", info.name);

                state.state = ConnectionState::InGame;
    
                controller.send_packet(oaPktS2XConnectionState {
                    state: OaPktS2xconnectionStateState::InGame,
                    ..Default::default()
                });
    
                let spawn_action = match controller.travel_mode() {
                    TravelMode::Login => ServerAction::DirectTravel(info.id, Some(movement.clone())),
                    TravelMode::EntryPoint => ServerAction::NonPortalTravel(info.id, Some(movement.clone())),
                    TravelMode::Portal { .. } => ServerAction::Portal(info.id, Some(movement.clone())), 
                    TravelMode::Position { .. } => ServerAction::DirectTravel(info.id, Some(movement.clone())),
                };
    
                controller.send_packet(spawn_action.into_pkt());
                commands.entity(ent).insert(Active);

                Ok(())
            } else {
                Err(anyhow!("Player not found!").into())
            }
        })?)?;

    player_api.set("ApplyClassItem", lua.create_bevy_function(world,
        |
            In((player, class_item, clear_inventory, callback)): In<(Table, String, bool, Option<Function>)>,
            query: Query<(Entity, &PlayerController)>,
            instance: Res<ZoneInstance>,
            systems: Res<PlayerSystems>,
            mut commands: Commands
        | -> WorldResult<()> {
            let (ent, controller) = query.get(player.entity()?)
                .map_err(|_| anyhow!("player not found"))?;

            let realm_api = instance.realm_api.clone();
            let character_id = controller.character_id();

            commands.run_system_async(async move {
                (ent, realm_api.character_apply_class_item(&character_id, &class_item, clear_inventory).await, callback)
            }, systems.apply_class_item_result);

            Ok(())
        })?)?;

    Ok(())
}

#[derive(Component)]
pub struct Skillbook(Vec<SkillbookEntry>);

#[derive(Component)]
pub struct EnableClientUpdates;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let (character_sender, character_receiver) = mpsc::channel::<(Entity, Character, Cooldowns, Skillbook)>(10);

        let player_systems = PlayerSystems {
            apply_class_item_result: app.register_system(apply_class_item_result),
        };

        app.insert_resource(ForeignResource(character_sender));
        app.insert_resource(ForeignResource(character_receiver));
        app.insert_resource(player_systems);

        app.add_systems(First, (
            request_player_characters,
            update_skillbook.before(insert_player_characters),
            insert_player_characters.before(init_gameobjects),
        ).run_if(in_state(InstanceState::Running)));

        app.add_systems(Update, spawn_player);

        app.add_systems(Last, (
            begin_loading_sequence,
            save_player_data.before(clear_obj_changes),
        ));
        
        app.register_message_handler(handle_avatar_update);

        app.register_command("instantKill", cmd_instant_kill);

        app.register_string_behavior(Class::Player, "respawnnow", behavior_respawnnow);

        insert_skillbook_api(app.world_mut()).unwrap();
        insert_player_api(app.world_mut()).unwrap();
    }
}

#[allow(clippy::type_complexity)]
fn request_player_characters(
    query: Query<(Entity, &PlayerController), Added<PlayerController>>,
    instance: Res<ZoneInstance>,
    sender: Res<ForeignResource<Sender<(Entity, Character, Cooldowns, Skillbook)>>>,
) {
    for (entity, controller) in query.iter() {
        let realm_api = instance.realm_api.clone();
        let sender = sender.clone();
    
        let state = controller.state().clone();
    
        instance.spawn_task(async move {
            if 
                let Ok(Some(mut character)) = realm_api.get_character(state.character()).await &&
                let Ok(mut skillbook) = realm_api.get_or_create_skillbook(*character.id()).await &&
                let Ok(ability_bar) = realm_api.get_or_create_ability_bar(*character.id()).await
            {
                let level = *character.data().get::<_, i32>(Player::Lvl).unwrap();
                let combat_style = CombatStyle::from_id(*character.data().get::<_, i32>(Player::CombatStyle).unwrap());

                if skillbook.combat_style != combat_style.into() {
                    debug!("Player combat style does not match skillbook");

                    if let Err(e) = skillbook.change_class(combat_style.into(), Some(level)).await {
                        warn!("Failed to change skillbook: {:?}", e);
                    }
                } else if skillbook.character_level != level {
                    let _ = skillbook.level_up(level).await;
                }

                let _ = skillbook.unlock_all().await;

                let skills = join_all(skillbook.skills.iter()
                    .map(async |s| {
                        if let Ok(Some(ability)) = OBJECT_CACHE.wait().get_object_by_guid(s.ability_id).await {
                            Some(SkillbookEntry(Arc::new(Skill {
                                id: s.id,
                                ability,
                                group: s.group.clone(),
                                state: s.state,
                                stance: s.stance,
                            })))
                        } else {
                            None
                        }
                    })
                ).await
                .into_iter()
                .flatten()
                .collect();
                

                let mut cooldowns = Cooldowns::default();

                // TODO: This is incredibly ugly. Cache cooldows on world start and copy them here or 
                // probably during player spawn.
                if let Ok(mut cursor) = realm_api.query_object_templates()
                    .class(Class::CooldownGroupExternal)
                    .query().await {
                    
                    while let Some(cooldown) = cursor.try_next().await.unwrap() {
                        let cooldown = OBJECT_CACHE.wait()
                            .get_object_by_guid(cooldown.id).await.unwrap()
                            .unwrap();

                        cooldowns.insert(cooldown);
                    }
                }

                let main_skill_bar = [()]
                    .repeat(7)
                    .into_iter()
                    .enumerate()
                    .map(|(i, _)| {
                        if let Some(entry) = ability_bar.slots.get(i) {
                            AbilityBarReference { id: entry.id, skill: entry.ability.clone() }
                        } else {
                            AbilityBarReference { id: -1, ..Default::default() }
                        }
                    })
                    .collect::<Vec<_>>();

                character.data_mut().set(Player::CurrentAbilityBarReferences, oaAbilityBarReferences {
                    class_hash: 0xFE0D0DC2,
                    version: 1,
                    count: main_skill_bar.len() as u32,
                    main_skill_bar,
                    single_slot_bar: AbilityBarReference {
                        id: ability_bar.single_slot.id,
                        skill: ability_bar.single_slot.ability,
                    },
                }.to_bytes());

                let _ = sender.send((entity, character, cooldowns, Skillbook(skills))).await;
            }
        });
    }
}

#[allow(clippy::type_complexity)]
fn insert_player_characters(
    mut receiver: ResMut<ForeignResource<Receiver<(Entity, Character, Cooldowns, Skillbook)>>>,
    controller: Query<&PlayerController>,
    starting_points: Query<&GameObjectData, With<StartingPointTag>>,
    portals: Query<(&ContentInfo, &GameObjectData), With<PortalTag>>,
    exit_nodes: Query<(&ContentInfo, &GameObjectData), With<SpawnNodeTag>>,
    instance: Res<ZoneInstance>,
    mut commands: Commands,
) {
    while let Ok((entity, mut character, cooldowns, skillbook)) = receiver.try_recv() {
        if let Ok(controller) = controller.get(entity) {

            // Update zone info in character data
            {
                let obj: &mut GameObjectData = character.data_mut();

                // First time spawn setup
                if *obj.get(Player::FirstTimeSpawn).unwrap() {
                    obj.set(Player::HpCur, obj.get::<_, Value>(Player::HpMax).unwrap().clone());

                    // Lookup the entrypoint
                    if let Some(starting_point) = starting_points.iter().next() {
                        obj.set(Player::Pos, (0u32, *starting_point.get::<_, Vec3>(NonClientBase::Pos).unwrap()));
                        obj.set(Player::Rot, *starting_point.get::<_, Vec3>(NonClientBase::Rot).unwrap());
                    } else {
                        error!("Starting point not found!");
                    }

                    //obj.set(Player::FirstTimeSpawn, false);

                    obj.set(Player::SpawnMode, 1);
                } else {
                    match controller.travel_mode() {
                        TravelMode::Login => {
                            obj.set(Player::SpawnMode, 2);
                        },
                        TravelMode::Portal { uuid } => {
                            if 
                                let Some((_, portal)) = portals.iter()
                                    .find(|(info, _)| info.placement_id == uuid) &&
                                let Some(exit_point_id) = portal.get::<_, String>(Portal::ExitPoint).ok()
                                    .and_then(|s| s.parse::<Uuid>().ok()) &&
                                let Some((_, exit_point)) = exit_nodes.iter()
                                    .find(|(info, _)| info.placement_id == exit_point_id)
                            {
                                obj.set(Player::Pos, (0u32, *exit_point.get::<_, Vec3>(NonClientBase::Pos).unwrap()));
                                obj.set(Player::Rot, *exit_point.get::<_, Vec3>(NonClientBase::Rot).unwrap());
                            } else {
                                warn!("No exit node found for portal {}", uuid);

                                // Lookup the entrypoint
                                if let Some(starting_point) = starting_points.iter().next() {
                                    obj.set(Player::Pos, (0u32, *starting_point.get::<_, Vec3>(NonClientBase::Pos).unwrap()));
                                    obj.set(Player::Rot, *starting_point.get::<_, Vec3>(NonClientBase::Rot).unwrap());
                                } else {
                                    error!("Starting point not found!");
                                }
                            }

                            obj.set(Player::SpawnMode, 4);
                        },
                        TravelMode::Position { pos, rot } => {
                            obj.set(Player::Pos, pos);
                            obj.set(Player::Rot, rot);
                            obj.set(Player::SpawnMode, 3);
                        },
                        TravelMode::EntryPoint => {
                            // Lookup the entrypoint
                            if let Some(starting_point) = starting_points.iter().next() {
                                obj.set(Player::Pos, (0u32, *starting_point.get::<_, Vec3>(NonClientBase::Pos).unwrap()));
                                obj.set(Player::Rot, *starting_point.get::<_, Vec3>(NonClientBase::Rot).unwrap());
                            } else {
                                error!("Starting point not found!");
                            }

                            obj.set(Player::SpawnMode, 3);
                        },
                    }
                }

                // Update stance data
                obj.set(Player::ClassData, oaPlayerClassData {
                    class_hash: 0x9D35021A,
                    ..Default::default()
                }.to_bytes());

                // Update zone info in player data
                obj.set(Player::WorldMapGuid, instance.world_def.guid().to_string());
                obj.set(Player::ZoneGuid, instance.zone.guid().to_string());
                obj.set(Player::InstanceZoneKey, instance.instance_id.map(|v| v.to_string()).unwrap_or_default());

                obj.set(Player::ClientReady, false);
                obj.set(Player::PlayerLoading, true);
            }

            let movement = Movement {
                position: character.data().get::<_, (u32, Vec3)>(Player::Pos).unwrap().1,
                rotation: Quat::from_unit_vector(*character.data().get::<_, Vec3>(Player::Rot).unwrap()),
                velocity: Vec3::ZERO,
                mode: PhysicsState::Walking,
                mover_type: 1,
                mover_replication_policy: 7,
                version: 0,
                mover_key: 0,
                seconds: 0.0,
            };

            // Insert character into world
            commands.entity(entity)
                .insert((
                    AvatarInfo {
                        id: controller.avatar_id(),
                        name: character.name().to_owned(),
                    },
                    character.take_data(),
                    movement,
                    QuestLog::default(),
                    cooldowns,
                    skillbook,
                ));
        }
    }
}

pub fn begin_loading_sequence(
    query: Query<(Entity, &PlayerController, &AvatarInfo, &GameObjectData, &Movement), Added<GameObjectData>>,
    mut commands: Commands,
) {
    for (ent, controller, avatar, obj, movement) in query.iter() {
        debug!("Starting spawning sequence for character: {}", avatar.name);

        let mut serialized = Vec::new();
        let mut writer = ByteWriter::endian(&mut serialized, LittleEndian);

        // Send character to client, so it begins loading the level
        if matches!(controller.travel_mode(), TravelMode::Login) {
            obj.write_to_privileged_client(&mut writer).unwrap();

            controller.send_packet(CPktBlob {
                avatar_id: controller.avatar_id(),
                avatar_name: avatar.name.clone(),
                class_id: obj.class().id() as u32,
                params: serialized.into(),
                movement: MoveManagerInit {
                    pos: movement.position.into(),
                    rot: movement.rotation.into(),
                    vel: movement.velocity.into(),
                    physics: Physics {
                        state: movement.mode,
                    },
                    mover_type: movement.mover_type,
                    mover_replication_policy: movement.mover_replication_policy,
                    version: movement.version,
                    ..Default::default()
                }.to_bytes().into(),
                has_guid: true,
                field_7: Some(*controller.session().id()),
                ..Default::default()
            });
        } else {
            let changes = obj.changes().
                filter(|(attr, _)| !attr.has_flag(&ParamFlag::ClientUnknown))
                .collect::<Box<dyn GenericParamSet>>();

            changes.write_to_privileged_client(&mut writer).unwrap();

            controller.send_packet(CPktAvatarUpdate {
                full_update: false,
                avatar_id: Some(controller.avatar_id()),
                name: Some(avatar.name.clone()),
                class_id: Some(obj.class().id() as u32),
                params: serialized.into(),
                ..Default::default()
            });
        }

        commands.entity(ent).insert(EnableClientUpdates);
    }
}

#[allow(clippy::type_complexity)]
fn spawn_player(
    mut query: Query<(&mut CurrentState, Option<&InitialInventoryTransfer>, &ScriptObject), Changed<CurrentState>>,
    instance: Res<ZoneInstance>,
    mut commands: Commands
) {
    for (state, inventory_transfer, obj) in query.iter_mut() {
        if 
            matches!(state.state, ConnectionState::InitialInterestsLoaded) &&
            inventory_transfer.is_none()
        {
            commands
                .entity(instance.world_controller)
                .call_named_lua_method("SpawnPlayer", obj.object().clone());
        }
    }
}

#[allow(clippy::type_complexity)]
fn save_player_data(
    query: Query<(&PlayerController, &GameObjectData), (Or<(Added<GameObjectData>, Changed<GameObjectData>)>, With<PlayerTag>)>,
    instance: Res<ZoneInstance>,
) {
    for (controller, obj) in query.iter() {
        let id = *controller.state().character();
        let volatile_diff = obj.changes()
        .filter(|(attr, _)| !attr.has_flag(&ParamFlag::Persistent))
            .collect::<Box<dyn GenericParamSet>>();
        let realm_api = instance.realm_api.clone();

        if 
            let Some(Value::Any(ability_bar)) = volatile_diff.get_param(Player::CurrentAbilityBarReferences.name()) &&
            let Ok((_, current_ability_bar)) = oaAbilityBarReferences::from_bytes(ability_bar)
        {
            let mut ability_bar = realm_api.create_empty_ability_bar(id);
            ability_bar.single_slot = AbilitySlot {
                id: current_ability_bar.single_slot_bar.id,
                ability: current_ability_bar.single_slot_bar.skill.clone(),
            };

            ability_bar.slots = current_ability_bar.main_skill_bar.iter()
                .map(|e| AbilitySlot {
                    id: e.id,
                    ability: e.skill.clone(),
                })
                .collect();

            instance.spawn_task(async move {
                let _ = ability_bar.save().await;
            });
        }

        let persistent_diff =  obj.changes()
            .filter(|(attr, _)| attr.has_flag(&ParamFlag::Persistent))
            .collect::<Box<dyn GenericParamSet>>();

        if !persistent_diff.is_empty() {
            trace!("Saving character update for: {} - {:#?}", id, persistent_diff);

            // We probably should move this into it's own task and just 
            // send a (blocking) message here, se we can have
            // backpressure in case our updates don't go trough.
            // Also, errors are not really handled here.
            instance.spawn_task(async move {
                if let Err(e) = realm_api.update_character_data_diff(&id, persistent_diff).await {
                    error!("Character update failed: {:?}", e);
                }
            });
        }  
    }
}

pub fn handle_avatar_update(
    In((ent, pkt)): In<(Entity, CPktAvatarUpdate)>,
    mut query: Query<(&PlayerController, &mut GameObjectData)>,
) {
    if 
        let Ok((controller, mut obj)) = query.get_mut(ent) &&
        let Ok((_, params)) = ParamSet::<Player>::from_slice(&pkt.params) &&

        // Ignore updates for any avatars other than the player avatar.
        pkt.avatar_id.unwrap_or_default() == controller.avatar_id() 
    {
        let mut params = params.into_iter()
            .filter(|(a, _)| !a.has_flag(&ParamFlag::ExcludeFromClient))
            .filter(|(a, v)| obj.get_named::<Value>(a.name()).unwrap() != v)
            .collect::<Box<dyn GenericParamSet>>();

        if 
            let Some(Value::Any(ability_bar)) = params.get_param(Player::CurrentAbilityBarReferences.name()) &&
            let Ok(ability_bar) = oaAbilityBarReferences::from_bytes(ability_bar)
        {
            debug!("{:#?}", ability_bar);
        }

        obj.apply(params.as_mut());
    }
}

fn cmd_instant_kill(
    In((ent, _)): In<(Entity, Vec<NativeParam>)>,
    mut event: EventWriter<HealthUpdateEvent>
) {
    event.send(HealthUpdateEvent::kill(ent));
}

#[allow(clippy::type_complexity)]
fn behavior_respawnnow(
    In((ent, _, behavior)): In<(Entity, Entity, StringBehavior)>,
    mut query: Query<(&PlayerController, &mut Movement), (With<PlayerTag>, Without<PortalTag>)>,
    portals: Query<&Movement, (With<PortalTag>, Without<PlayerTag>)>,
    mut event: EventWriter<HealthUpdateEvent>
) {
    let mode = behavior.args.first();

    match mode.map(|s| s.as_str()) {
        Some("NearestPortal") => {
            if let Ok((controller, _)) = query.get_mut(ent) {
                event.send(HealthUpdateEvent::revive(ent, None));

                if let Some(pos) = portals.iter().next() {
                    controller.send_packet(
                        ServerAction::LocalPortal(controller.avatar_id(), pos.clone()).into_pkt()
                    );
                }
            }
        },
        Some(m) => warn!("Unknown respawn mode: {}", m),
        None => (),
    }
}

#[allow(clippy::type_complexity)]
fn update_skillbook(
    mut query: Query<(&mut GameObjectData, &Skillbook), Changed<Skillbook>>,
) {
    for (mut player, skillbook) in query.iter_mut() {
        debug!("Updating skillbook");

        player.set(Player::CurrentClassSkills,
            oaAbilityDataPlayerArray {
                class_hash: 0x81E0A735,
                count: skillbook.0.len() as u32,
                skills: skillbook.0.iter()
                    .map(|s| oaAbilityDataPlayer {
                        version: 0,
                        id: s.id,
                        content_id: s.ability.id,
                        group: s.group.clone(),
                        field_4: s.stance,
                    })
                    .collect(),
            }.to_bytes());
    }
}

#[derive(Resource)]
#[allow(clippy::type_complexity)]
struct PlayerSystems {
    apply_class_item_result: SystemId<In<(Entity, RealmApiResult<EquipmentResult>, Option<Function>)>>,
}

fn apply_class_item_result(
    In((ent, result, callback)): In<(Entity, RealmApiResult<EquipmentResult>, Option<Function>)>,
    mut query: Query<&mut GameObjectData>,
    mut commands: Commands,
) {
    match result {
        Ok(res) => {
            if let Some(mut changes) = res.character_update {
                if let Ok(mut data) = query.get_mut(ent) {
                    data.apply(changes.as_mut());
                } else {
                    error!("Player not found!");
                }

                if let Some(callback) = callback {
                    commands
                        .entity(ent)
                        .call_lua_method(callback, ());
                } else {
                    debug!("No callback")
                }
            }
        },
        Err(e) => {
            error!("Failed to apply class item: {:?}", e);
            commands
                .entity(ent)
                .despawn_recursive();
        }
    }
}
