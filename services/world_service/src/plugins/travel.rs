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

use bevy::{app::Plugin, prelude::{App, Component, Entity, In, Query, Res}};
use futures_util::TryStreamExt;
use log::error;
use realm_api::{RealmApi, ZoneType};
use toolkit::{types::AvatarId, IterExt, NativeParam};

use crate::{error::{WorldError, WorldResult}, instance::ZoneInstance, proto::TravelMode};

use super::{CommandExtPriv, CommandMessage, NetworkExtPriv, PlayerController};

pub struct TravelPlugin;

impl Plugin for TravelPlugin {
    fn build(&self, app: &mut App) {
        app.register_community_command_handler(handle_join_dungeon);
        app.register_community_command_handler(handle_leave_dungeon);
        app.register_community_command_handler(handle_social_travel);

        app.register_command("teleport_to_world", |
            In((ent, args)): In<(Entity, Vec<NativeParam>)>,
            players: Query<&PlayerController>,
            instance: Res<ZoneInstance>,
        | {
            let mut args = args.into_iter();

            if 
                let Some(NativeParam::String(world)) = args.next() &&
                let Ok(controller) = players.get(ent).cloned() 
            {
                instance.spawn_task(async move {
                    if 
                    let Some(world_def) = RealmApi::get()
                        .query_worlddefs()
                        .name(world.clone())
                        .query().await?.try_next().await? &&
                    let Some(zone) = RealmApi::get()
                        .query_zones()
                        .zone_type(ZoneType::World)
                        .worlddef_guid(*world_def.guid())
                        .query().await?.try_next().await?
                {
                    controller.request_travel(*zone.guid(), None, TravelMode::EntryPoint, None);
                } else {
                    error!("Map '{world}' not found!");
                }

                Ok::<_, WorldError>(())
                });
            }
        });
    }
}

#[derive(Component)]
// Mark players who are currently in the process of travelling
// away from this zone.
pub struct Travelling;

#[allow(dead_code)]
struct JoinDungeon {
    avatar: AvatarId,
    map_name: String,
    flag: i32,
}

impl CommandMessage for JoinDungeon {
    fn id() -> i32 { 0x31 }

    fn from_native_param(data: NativeParam) -> WorldResult<Self> {
        let mut values = data.to_struct_iter()?.skip(1);

        Ok(Self {
            avatar: values.try_next()?.to_avatar_id()?, 
            map_name: values.try_next()?.to_string()?, 
            flag: values.try_next()?.to_i32()?
        })
    }
}

fn handle_join_dungeon(
    In((ent, cmd)): In<(Entity, JoinDungeon)>,
    instance: Res<ZoneInstance>,
    players: Query<&PlayerController>,
) {
    if let Ok(controller) = players.get(ent).cloned() {
        instance.spawn_task(async move {
            if let Err(e) = async {
                
                if 
                    let Some(world_def) = RealmApi::get()
                        .query_worlddefs()
                        .name(cmd.map_name.clone())
                        .query().await?.try_next().await? &&
                    let Some(zone) = RealmApi::get()
                        .query_zones()
                        .zone_type(ZoneType::World)
                        .worlddef_guid(*world_def.guid())
                        .query().await?.try_next().await?
                {
                    controller.request_travel(*zone.guid(), None, TravelMode::EntryPoint, None);
                } else {
                    error!("Map '{}' not found!", cmd.map_name);
                }

                Ok::<_, WorldError>(())
            }.await {
                error!("Failed to travel to map '{}': {:?}", cmd.map_name, e);
            }
        });
    }
}

#[allow(dead_code)]
struct LeaveDungeon {
    avatar: AvatarId,
    boolean: bool,
}

impl CommandMessage for LeaveDungeon {
    fn id() -> i32 { 0x35 }

    fn from_native_param(data: NativeParam) -> WorldResult<Self> {
        let mut values = data.to_struct_iter()?.skip(1);

        Ok(Self {
            avatar: values.try_next()?.to_avatar_id()?, 
            boolean: values.try_next()?.to_bool()?, 
        })
    }
}

fn handle_leave_dungeon(
    In((ent, _)): In<(Entity, LeaveDungeon)>,
    instance: Res<ZoneInstance>,
    players: Query<&PlayerController>,
) {
    if instance.config.json_config["IsTutorial"].as_bool().unwrap_or_default() {
        if let Ok(controller) = players.get(ent).cloned() {
            controller.request_travel("4635f288-ec24-4e73-b75c-958f2607a30e".parse().unwrap(), None, TravelMode::EntryPoint, None);
        }
    } else {
        // Where do we go?
        todo!()
    }
}

#[allow(dead_code)]
struct SocialTravel {
    avatar: AvatarId, 
    map_name: String, 
    is_travel: bool
}

impl CommandMessage for SocialTravel {
    fn id() -> i32 { 0xb3 }

    fn from_native_param(data: NativeParam) -> WorldResult<Self> {
        let mut values = data.to_struct_iter()?.skip(1);

        Ok(Self {
            avatar: values.try_next()?.to_avatar_id()?, 
            map_name: values.try_next()?.to_string()?, 
            is_travel: values.try_next()?.to_bool()?, 
        })
    }
}

fn handle_social_travel(
    In((ent, cmd)): In<(Entity, SocialTravel)>,
    instance: Res<ZoneInstance>,
    players: Query<&PlayerController>,
) {
    if 
        cmd.is_travel &&
        let Ok(controller) = players.get(ent).cloned()
    {
        instance.spawn_task(async move {
            if let Err(e) = async {
                
                if 
                    let Some(world_def) = RealmApi::get()
                        .query_worlddefs()
                        .name(cmd.map_name.clone())
                        .query().await?.try_next().await? &&
                    let Some(zone) = RealmApi::get()
                        .query_zones()
                        .zone_type(ZoneType::World)
                        .worlddef_guid(*world_def.guid())
                        .query().await?.try_next().await?
                {
                    controller.request_travel(*zone.guid(), None, TravelMode::EntryPoint, None);
                } else {
                    error!("Map '{}' not found!", cmd.map_name);
                }

                Ok::<_, WorldError>(())
            }.await {
                error!("Failed to travel to map '{}': {e:?}", cmd.map_name);
            }
        });
    }
}
