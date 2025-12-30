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

use bevy::ecs::{system::{In, Query}, world::World};
use mlua::{Lua, Table};
use protocol::{oaPktPortalRequestAck, PortalAckPartB};
use scripting::{LuaExt, LuaRuntime, LuaTableExt, ScriptResult};
use anyhow::anyhow;
use toolkit::NativeParam;

use crate::{error::WorldResult, plugins::{Avatar, PlayerController}};

pub fn insert_portalbook_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let object_api = lua.create_table().unwrap();
    runtime.register_native("portalbook", object_api.clone()).unwrap();

    object_api.set("Send", lua.create_bevy_function(world,         |
        In(portal_book): In<Table>,
        avatars: Query<&Avatar>,
        query: Query<&PlayerController>,
    | -> WorldResult<()> {
        let player = portal_book.get::<Table>("player")?.entity()?;
        let portal = portal_book.get::<Table>("portal")?.entity()?;

        if let Ok(ctrl) = query.get(player) {
            let portals = portal_book.get::<Table>("portals")?
                .sequence_values::<Table>()
                .map(|v| {
                    let v = v?;
                    Ok(PortalAckPartB {
                        map_name: v.get::<String>("map_name")?,
                        world_name: v.get::<String>("world_name")?,
                        display_name: v.get::<String>("display_name")?,
                        world_icon: v.get::<String>("world_icon")?,
                        portal_icon: v.get::<String>("portal_icon")?,
                        description: v.get::<String>("description")?,
                        portal_type: v.get::<u32>("portal_type")?,
                        world_type: v.get::<u32>("world_type")?,
                        player_level: v.get::<u32>("player_level")?,
                        level: v.get::<u32>("level")?,
                    })
                })
                .collect::<mlua::Result<Vec<PortalAckPartB>>>()?;

            ctrl.send_packet(oaPktPortalRequestAck {
                avatar_id: ctrl.avatar_id(),
                portal_id: avatars
                    .get(portal)
                    .map_err(|_| anyhow!("portal not found"))?.id,
                array_len_a: portals.len() as u32,
                field_5: portals,

                field_12: NativeParam::Struct(vec![
                    NativeParam::String(portal_book.get::<String>("name")?),
                    NativeParam::Bool(portal_book.get::<bool>("can_save")?),
                    NativeParam::Int(0),
                    NativeParam::Int(portal_book.get::<i32>("max_slots")?),
                ]),
                ..Default::default()
            });
        }

        Ok(())
    })?)?;
    
    Ok(())
}