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

use std::{ops::Div, sync::Mutex};

use anyhow::anyhow;
use bevy::{app::{Plugin, PostUpdate, Update}, ecs::{component::Component, entity::Entity, event::EventReader, lifecycle::RemovedComponents, query::{Changed, With}, resource::Resource, schedule::IntoScheduleConfigs, system::{Commands, In, Query, Res}, world::World}, math::{Quat, Vec3, Vec3A, bounding::Aabb3d}, time::{Time, Virtual}};
use bitstream_io::{ByteWrite, ByteWriter, LittleEndian};
use futures::{TryStreamExt};
use log::{debug, error};
use mlua::{Lua, Table};
use obj_params::{tags::NonClientBaseTag, GameObjectData, NonClientBase};
use protocol::{CPktAvatarBehaviors, NetworkVec3};
use realm_api::{RealmApi, WorldDef};
use recastnavigation_rs::{detour::{DtBuf, DtNavMesh, DtNavMeshParams, DtNavMeshQuery, DtPolyRef, DtQueryFilter, DtTileRef}, detour_crowd::DtPathCorridor};
use scripting::{LuaExt, LuaRuntime, LuaTableExt, ScriptResult};
use toolkit::{bson, OtherlandQuatExt, Vec3Wrapper};

use crate::{error::{WorldError, WorldResult}, plugins::{Active, Avatar, InterestTransmitted, Interests, Movement, PlayerController}};

pub struct NavigationPlugin;


impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (
                set_agent_targets,
                update,
            ).chain()
        );

        app.add_systems(PostUpdate, (
            replicate_paths_on_clients,
            replicate_changed_paths_on_clients,
            cleanup_targets,
        ));

        insert_navigation_api(app.world_mut()).unwrap();
    }
}

fn insert_navigation_api(
    world: &mut World,
) -> ScriptResult<()> {
    let runtime = world.get_resource::<LuaRuntime>().unwrap();
    let lua: Lua = runtime.vm().clone();
    let navigation_api = lua.create_table().unwrap();
    runtime.register_native("navigation", navigation_api.clone()).unwrap();

    navigation_api.set("MoveToPosition", lua.create_bevy_function(world, 
        |
            In((obj, location, speed, callback)): In<(Table, Vec3Wrapper, f32, mlua::Function)>,
            mut commands: Commands,
        | -> WorldResult<()> {
            let obj = obj.entity()?;

            commands
                .entity(obj)
                .insert(NavTarget {
                    pos: location.0,
                    speed,
                    callback: Some(callback),
                });

            Ok(())
        })?)?;

    navigation_api.set("CancelMovement", lua.create_bevy_function(world, 
        |
            In(obj): In<Table>,
            query: Query<&Movement, With<NavTarget>>,
            mut commands: Commands,
            time: Res<Time<Virtual>>,
        | -> WorldResult<()> {
            let obj = obj.entity()?;

            if let Ok(movement) = query.get(obj) {
                commands
                    .entity(obj)
                    .remove::<NavTarget>()
                    .remove::<PathCorridor>()
                    .insert(Pathing {
                        start_pos: movement.position.into(),
                        start_time: time.elapsed_secs(),
                        anchor_pos: movement.position.into(),
                        is_abort_early: Some(true),
                        ..Default::default()
                    });
            }

            Ok(())
        })?)?;

    Ok(())
}

struct SendDtNavMesh(DtNavMesh);

unsafe impl Send for SendDtNavMesh {}
unsafe impl Sync for SendDtNavMesh {}

impl std::ops::Deref for SendDtNavMesh {
    type Target = DtNavMesh;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SendDtNavMesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct SendDtNavMeshQuery(DtNavMeshQuery);

unsafe impl Send for SendDtNavMeshQuery {}
unsafe impl Sync for SendDtNavMeshQuery {}

impl std::ops::Deref for SendDtNavMeshQuery {
    type Target = DtNavMeshQuery;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SendDtNavMeshQuery {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct RecastNav {
    mesh: SendDtNavMesh,
    query: SendDtNavMeshQuery,
}

#[derive(Resource)]
pub struct Navmesh {
    recast: Mutex<RecastNav>,
    federation: realm_api::Navmesh,
    filter: DtQueryFilter,
}

const PATHENGINE_SCALE_FACTOR: f32 = 1.4305115;

impl Navmesh {
    pub fn get_pathengine_tile_for_pos(&self, pos: Vec3) -> i32 {

        ((pos.z * PATHENGINE_SCALE_FACTOR).ceil() as i32 - self.federation.pathengine_start_x).div(self.federation.pathengine_tile_size) + 
        ((pos.x * PATHENGINE_SCALE_FACTOR).ceil() as i32 - self.federation.pathengine_start_y).div(self.federation.pathengine_tile_size) * self.federation.pathengine_tile_pitch
    }

    pub fn get_floor_height(&self, pos: Vec3) -> Option<f32> {
        let recast = self.recast.lock().unwrap();

        if 
            let Ok((poly, pos)) = recast.query.find_nearest_poly_1(&Vec3::new(pos.x, pos.y, pos.z).to_array(), &[10.0, 1000.0, 10.0], &self.filter) &&
            let Ok(height) = recast.query.get_poly_height(poly, &pos)
        {
            Some(height)
        } else {
            None
        }
    }
}

impl Navmesh {
    pub async fn load(realm_api: RealmApi, world: &WorldDef) -> WorldResult<Self> {
        let navmesh = realm_api
            .query_navmeshs()
            .world_guid(*world.guid())
            .query()
            .await?
            .try_next()
            .await?
            .ok_or(WorldError::Other(anyhow!("No navigation mesh found for world: {}", world.guid())))?;

        let tiles = realm_api
            .query_navmesh_tiles()
            .mesh_id(navmesh.id)
            .query()
            .await?
            .try_collect::<Vec<_>>()
            .await?;

        let params = DtNavMeshParams {
            orig: [navmesh.origin[0] as f32, navmesh.origin[1] as f32, navmesh.origin[2] as f32],
            tile_height: navmesh.tile_height as f32,
            tile_width: navmesh.tile_width as f32,
            max_tiles: 1 << 7,
            max_polys: 1 << 15,
        };

        let mut mesh = DtNavMesh::with_params(&params)?;
        
        for tile in &tiles {
            let data = DtBuf::copy(&bson::Binary::from_base64(&tile.data, None)?.bytes)?;
            mesh.add_tile(data, DtTileRef::default())?;
        }

        debug!("Loaded {} tiles for map {}", tiles.len(), world.name());

        debug!("Max tiles: {}", mesh.max_tiles());

        {
            if let Some(tile) = mesh.get_tile(0) {
                debug!("Header: {:#?}", tile.header());
            }
        }

        let mut query = DtNavMeshQuery::new();
        query.init(&mesh, 2048)?;

        Ok(Self {
            recast: Mutex::new(RecastNav {
                mesh: SendDtNavMesh(mesh),
                query: SendDtNavMeshQuery(query),
            }),
            federation: navmesh,
            filter: DtQueryFilter {
                include_flags: 0xFFFF,
                ..Default::default()
            },
        })
    }

    pub fn bounds(&self) -> Aabb3d {
        let recast = self.recast.lock().unwrap();
        let mut bounds = Aabb3d::new(Vec3A::default(), Vec3A::default());
        
        for tile in 0..recast.mesh.max_tiles() {
            if 
                let Some(tile) = recast.mesh.get_tile(tile) &&
                let Some(header) = tile.header()
            {
                bounds.min.x = bounds.min.x.min(header.bmin[0]);
                bounds.min.y = bounds.min.y.min(header.bmin[1]);
                bounds.min.z = bounds.min.z.min(header.bmin[2]);
                bounds.max.x = bounds.max.x.max(header.bmax[0]);
                bounds.max.y = bounds.max.y.max(header.bmax[1]);
                bounds.max.z = bounds.max.z.max(header.bmax[2]);
            }
        }

        bounds
    }
}

#[derive(Component)]
pub struct PathCorridor {
    corridor: DtPathCorridor,
    segment: Option<PathSegment>,
}

struct PathSegment {
    start: Vec3,
    end: Vec3,
    duration: f32,
    elapsed: f32,
    speed: f32,
}

pub fn update(
    navmesh: Res<Navmesh>,
    time: Res<Time<Virtual>>,
    mut query: Query<(Entity, &mut Movement, &GameObjectData, &NavTarget, &mut PathCorridor), With<PathCorridor>>,
    mut commands: Commands,
) {
    let mut recast = navmesh.recast.lock().unwrap();

    for (ent, mut movement, data, target, mut corridor) in query.iter_mut() {
        match corridor.segment.as_mut() {
            Some(lerp) => {
                // Send pathing update to the client
                if lerp.elapsed == 0.0 {
                    commands.entity(ent)
                        .insert(Pathing {
                            start_time: time.elapsed_secs(),
                            start_pos: lerp.start.into(),
                            anchor_pos: lerp.end.into(),
                            force_find_path: Some(false),
                            speed: lerp.speed,
                            acceleration: lerp.speed,
                            mover_key: movement.mover_key,
                            //forced_movement_mode: Some(0),
                            traverse_costs: Some(1),
                            client_dont_care: Some(false),
                            backward: Some(false),
                            face_target: Some(true),
                            target_tile: Some(navmesh.get_pathengine_tile_for_pos(lerp.end)),
                            is_abort_early: Some(true),
                            keep_z_value: Some(false),
                            while_obstructed: Some(true),
                            ..Default::default()
                        });
                }

                lerp.elapsed = (lerp.elapsed + time.delta_secs()).clamp(0.0, lerp.duration);

                movement.position = lerp.start.lerp(lerp.end, lerp.elapsed / lerp.duration);
                movement.rotation = Quat::from_unit_vector((lerp.end.with_y(0.0) - lerp.start.with_y(0.0)).normalize());
                movement.seconds = time.elapsed_secs_f64();

                if lerp.elapsed >= lerp.duration {
                    debug!("Reached end of path segment for entity {ent}");
                    corridor.segment = None; 
                }

                if let Some(callback) = &target.callback {
                    let _ = callback.call::<()>(("PATH_SEGMENT_COMPLETE", Vec3Wrapper(movement.position)));
                }
            },
            None => {
                debug!("Update path position for entity {ent} at position: {}", movement.position);

                if !corridor.corridor.move_position(&movement.position.to_array(), &mut recast.query, &navmesh.filter) {
                    if let Some(callback) = &target.callback {
                        let _ = callback.call::<()>(("INVALID_POSITION", Vec3Wrapper(target.pos)));
                    }

                    debug!("Failed to move position for entity {ent}, removing PathCorridor");
                    commands.entity(ent)
                        .remove::<PathCorridor>()
                        .remove::<NavTarget>();
                    continue;
                }

                let mut corner_verts = vec![[0.0f32; 3]; MAX_CORNERS];
                let mut corner_flags = vec![0u8; MAX_CORNERS];
                let mut corner_polys = vec![DtPolyRef::default(); MAX_CORNERS];

                corridor.corridor.optimize_path_topology(&mut recast.query, &navmesh.filter);

                let corners = corridor.corridor.find_corners(&mut corner_verts, &mut corner_flags, &mut corner_polys, &mut recast.query, &navmesh.filter) as usize;

                corner_verts.truncate(corners);
                corner_flags.truncate(corners);
                corner_polys.truncate(corners);

                debug!("Corners: {corner_verts:?}");

                if let Some(next) = corner_verts.first() {
                    let start = Vec3::from_slice(corridor.corridor.pos());
                    let next = Vec3::from_slice(next);
                    let end = Vec3::from_slice(corridor.corridor.target());
                    let duration = start.distance(next) / target.speed;

                    debug!("Entity {ent} path corridor updated: Start: {start:?}, Next: {next:?} End: {end:?}, Duration: {duration}, Speed: {}", target.speed);

                    if start.with_y(0.0).distance(end.with_y(0.0)) < 15.0 && *corner_polys.last().unwrap() == DtPolyRef(0) {
                        if let Some(callback) = &target.callback {
                            let _ = callback.call::<()>(("FINISHED", Vec3Wrapper(target.pos)));
                        }

                        debug!("Start and end positions are too close for entity {ent}, removing PathCorridor");
                        commands.entity(ent)
                            .remove::<PathCorridor>()
                            .remove::<NavTarget>();
                        continue;
                    }

                    corridor.segment = Some(PathSegment { 
                        start, 
                        end: next, 
                        duration,
                        elapsed: 0.0,
                        speed: target.speed,
                    });
                } else {
                    if let Some(callback) = &target.callback {
                        let _ = callback.call::<()>(("FINISHED", Vec3Wrapper(target.pos)));
                    }

                    debug!("No corners found for entity {ent}, removing PathCorridor");
                    commands.entity(ent)
                        .remove::<PathCorridor>()
                        .remove::<NavTarget>();
                    continue;
                }
            }

        }
    }
}

#[derive(Component)]
pub struct NavTarget {
    pos: Vec3,
    speed: f32,
    callback: Option<mlua::Function>,
}

const MAX_CORNERS: usize = 2;

#[allow(clippy::type_complexity)]
fn set_agent_targets(
    mut query: Query<(Entity, &GameObjectData, &Movement, &NavTarget, Option<&mut PathCorridor>), Changed<NavTarget>>,
    navmesh: Res<Navmesh>,
    mut commands: Commands,
) {
    let mut recast = navmesh.recast.lock().unwrap();

    for (ent, _obj, movement, target, corridor) in query.iter_mut() {
        let start_pos = movement.position.to_array();
        let target_pos = target.pos.to_array();

        debug!("Target pos: {target_pos:?}");

        if 
            let Some(mut corridor) = corridor &&
            corridor.corridor.move_target_position(&target_pos, &mut recast.query, &navmesh.filter)
        {
            let new_pos = Vec3::from_slice(corridor.corridor.target());

            // New position is within the corridor
            if new_pos.distance(target.pos) < 10.0 {
                corridor.segment = None; // Clear segment to update path
                continue;
            }
        }

        //let mut path = vec![DtPolyRef::default(); 256];
        let mut corridor = DtPathCorridor::default();
        if !corridor.init(256) {
            error!("Failed to initialize path corridor for entity: {ent}");
            continue;
        }

        // Lookup navmesh tile
        if 
            let Ok((start_ref, start_pos_poly)) = recast.query.find_nearest_poly_1(&start_pos, &[100.0, 100.0, 100.0], &navmesh.filter) &&
            let Ok((end_ref, end_pos_poly)) = recast.query.find_nearest_poly_1(&target_pos, &[100.0, 100.0, 100.0], &navmesh.filter)
        {
            debug!("Found start poly: {start_ref:?} at position {start_pos_poly:?}");
            debug!("Found end poly: {end_ref:?} at position {end_pos_poly:?}");

            let mut path = vec![DtPolyRef::default(); 256];

            match recast.query.find_path(start_ref, end_ref, &start_pos_poly, &end_pos_poly, &navmesh.filter, &mut path) {
                Ok(path_size) => {
                    corridor.reset(start_ref, &start_pos_poly);
                    corridor.set_corridor(&end_pos_poly, &path, path_size as i32);

                    commands.entity(ent)
                        .insert(PathCorridor {
                            corridor,
                            segment: None,
                        });

                    if let Some(callback) = &target.callback {
                        let _ = callback.call::<()>(("FOUND_CORRIDOR", Vec3Wrapper(target.pos)));
                    }
                },
                Err(e) => {
                    error!("Failed to find path from {start_pos:?} to {target_pos:?} for entity: {ent} Error: {e}");
                    if let Some(callback) = &target.callback {
                        let _ = callback.call::<()>(("PATHFINDING_FAILED", Vec3Wrapper(target.pos)));
                    }
                }
            }
        } else {
            error!("Failed to find nearest poly for target: {:?}", target.pos);
            if let Some(callback) = &target.callback {
                let _ = callback.call::<()>(("TARGET_NOT_FOUND", Vec3Wrapper(target.pos)));
            }
        }
    }
}

#[derive(Component, Clone, Default)]
pub struct Pathing {
    pub start_time: f32,
    pub backward: Option<bool>,
    pub face_target: Option<bool>,
    pub anchor_pos: NetworkVec3,
    pub start_pos: NetworkVec3,
    pub keep_z_value: Option<bool>,
    pub force_find_path: Option<bool>,
    pub is_abort_early: Option<bool>,
    pub traverse_costs: Option<i32>,
    pub speed: f32,
    pub acceleration: f32,
    pub client_dont_care: Option<bool>,
    pub forced_movement_mode: Option<i32>,
    pub target_tile: Option<i32>,
    pub while_obstructed: Option<bool>,
    pub mover_key: u16,
}

impl Pathing {
    fn to_bytes(&self) -> WorldResult<Vec<u8>> {
        let mut data = Vec::new();
        let mut writer = ByteWriter::endian(&mut data, LittleEndian);

        writer.write(1u16)?;

        writer.write(0xE7AB83B2u32)?; // PathEngineMove

        // Parameters
        writer.write(self.backward.unwrap_or_default() as u8)?; // backward
        writer.write(self.face_target.unwrap_or_default() as u8)?; // faceTarget
        writer.write_bytes(&self.anchor_pos.to_bytes())?; // anchorPos
        writer.write_bytes(&self.start_pos.to_bytes())?; // startingPos
        writer.write(self.keep_z_value.unwrap_or_default() as u8)?; // keepZValue
        writer.write(self.force_find_path.unwrap_or_default() as u8)?; // ForceFindPath
        writer.write(self.is_abort_early.unwrap_or_default() as u8)?; // isAbortEarly
        writer.write(self.traverse_costs.unwrap_or(0))?; // traverseCosts
        writer.write(self.start_time)?; // startTime
        writer.write(self.speed)?; // speed
        writer.write(self.acceleration)?; // accel
        writer.write(self.client_dont_care.unwrap_or_default() as u8)?; // clientDontCare
        writer.write(self.forced_movement_mode.unwrap_or_default())?; // forcedMovementMode
        writer.write(self.target_tile.unwrap_or(0))?; // targetTile
        writer.write(self.while_obstructed.unwrap_or_default() as u8)?; // whileObstructed
        writer.write(self.mover_key)?; // moverKey

        writer.write(0i32)?; // Unknown

        Ok(data)
    }
}

#[allow(dead_code)]
// This function sadly doesn't produce the correct output. 
// If this could be fixed, the constant used in `replicate_path_on_clients` could be replaced by this.
fn factory_name_hash(input: &[u8]) -> u32 {
    if input.is_empty() {
        return 0;
    }
    
    // CRC-16-CCITT lookup table (polynomial 0x1021)
    const CRC_TABLE: [u16; 256] = [
        0x0000, 0x1021, 0x2042, 0x3063, 0x4084, 0x50A5, 0x60C6, 0x70E7,
        0x8108, 0x9129, 0xA14A, 0xB16B, 0xC18C, 0xD1AD, 0xE1CE, 0xF1EF,
        0x1231, 0x0210, 0x3273, 0x2252, 0x52B5, 0x4294, 0x72F7, 0x62D6,
        0x9339, 0x8318, 0xB37B, 0xA35A, 0xD3BD, 0xC39C, 0xF3FF, 0xE3DE,
        0x2462, 0x3443, 0x0420, 0x1401, 0x64E6, 0x74C7, 0x44A4, 0x5485,
        0xA56A, 0xB54B, 0x8528, 0x9509, 0xE5EE, 0xF5CF, 0xC5AC, 0xD58D,
        0x3653, 0x2672, 0x1611, 0x0630, 0x76D7, 0x66F6, 0x5695, 0x46B4,
        0xB75B, 0xA77A, 0x9719, 0x8738, 0xF7DF, 0xE7FE, 0xD79D, 0xC7BC,
        0x48C4, 0x58E5, 0x6886, 0x78A7, 0x0840, 0x1861, 0x2802, 0x3823,
        0xC9CC, 0xD9ED, 0xE98E, 0xF9AF, 0x8948, 0x9969, 0xA90A, 0xB92B,
        0x5AF5, 0x4AD4, 0x7AB7, 0x6A96, 0x1A71, 0x0A50, 0x3A33, 0x2A12,
        0xDBFD, 0xCBDC, 0xFBBF, 0xEB9E, 0x9B79, 0x8B58, 0xBB3B, 0xAB1A,
        0x6CA6, 0x7C87, 0x4CE4, 0x5CC5, 0x2C22, 0x3C03, 0x0C60, 0x1C41,
        0xEDAE, 0xFD8F, 0xCDEC, 0xDDCD, 0xAD2A, 0xBD0B, 0x8D68, 0x9D49,
        0x7E97, 0x6EB6, 0x5ED5, 0x4EF4, 0x3E13, 0x2E32, 0x1E51, 0x0E70,
        0xFF9F, 0xEFBE, 0xDFDD, 0xCFFC, 0xBF1B, 0xAF3A, 0x9F59, 0x8F78,
        0x9188, 0x81A9, 0xB1CA, 0xA1EB, 0xD10C, 0xC12D, 0xF14E, 0xE16F,
        0x1080, 0x00A1, 0x30C2, 0x20E3, 0x5004, 0x4025, 0x7046, 0x6067,
        0x83B9, 0x9398, 0xA3FB, 0xB3DA, 0xC33D, 0xD31C, 0xE37F, 0xF35E,
        0x02B1, 0x1290, 0x22F3, 0x32D2, 0x4235, 0x5214, 0x6277, 0x7256,
        0xB5EA, 0xA5CB, 0x95A8, 0x8589, 0xF56E, 0xE54F, 0xD52C, 0xC50D,
        0x34E2, 0x24C3, 0x14A0, 0x0481, 0x7466, 0x6447, 0x5424, 0x4405,
        0xA7DB, 0xB7FA, 0x8799, 0x97B8, 0xE75F, 0xF77E, 0xC71D, 0xD73C,
        0x26D3, 0x36F2, 0x0691, 0x16B0, 0x6657, 0x7676, 0x4615, 0x5634,
        0xD94C, 0xC96D, 0xF90E, 0xE92F, 0x99C8, 0x89E9, 0xB98A, 0xA9AB,
        0x5844, 0x4865, 0x7806, 0x6827, 0x18C0, 0x08E1, 0x3882, 0x28A3,
        0xCB7D, 0xDB5C, 0xEB3F, 0xFB1E, 0x8BF9, 0x9BD8, 0xABBB, 0xBB9A,
        0x4A75, 0x5A54, 0x6A37, 0x7A16, 0x0AF1, 0x1AD0, 0x2AB3, 0x3A92,
        0xFD2E, 0xED0F, 0xDD6C, 0xCD4D, 0xBDAA, 0xAD8B, 0x9DE8, 0x8DC9,
        0x7C26, 0x6C07, 0x5C64, 0x4C45, 0x3CA2, 0x2C83, 0x1CE0, 0x0CC1,
        0xEF1F, 0xFF3E, 0xCF5D, 0xDF7C, 0xAF9B, 0xBFBA, 0x8FD9, 0x9FF8,
        0x6E17, 0x7E36, 0x4E55, 0x5E74, 0x2E93, 0x3EB2, 0x0ED1, 0x1EF0,
    ];
    
    let mut crc_buffers = [0xFFFFu16; 2];
    
    for (counter, &byte) in input.iter().enumerate() {
        let buffer_idx = counter & 1;
        let current_crc = crc_buffers[buffer_idx];

        let table_index = (byte ^ (current_crc >> 8) as u8) as usize;
        crc_buffers[buffer_idx] = CRC_TABLE[table_index] ^ (current_crc << 8);
    }

    (crc_buffers[0] as u32) << 16 | (crc_buffers[1] as u32)
}

fn replicate_changed_paths_on_clients(
    agents: Query<(Entity, &Avatar, &Pathing), Changed<Pathing>>,
    clients: Query<(&Interests, &PlayerController)>,
) {
    for (ent, info, pathing) in agents.iter() {
        let data = pathing.to_bytes()
            .expect("Failed to serialize Pathing data");

        for (interests, controller) in clients.iter() {
            if interests.contains_key(&ent) {
                debug!("Replicating pathing data for agent {} to client {}", info.id, controller.avatar_id());

                controller.send_packet(CPktAvatarBehaviors {
                    field_1: info.id,
                    field_2: data.clone().into(),
                    ..Default::default()
                });
            }
        }
    }
}

fn replicate_paths_on_clients(
    mut transmitted_interests: EventReader<InterestTransmitted>,
    agents: Query<(&Avatar, &Pathing, &Movement)>,
    clients: Query<&PlayerController>,
    time: Res<Time<Virtual>>,
) {
    for InterestTransmitted(target, ent) in transmitted_interests.read() {
        if 
            let Ok(controller) = clients.get(*target) &&
            let Ok((info, pathing, movement)) = agents.get(*ent)
        {
            debug!("Replicating pathing data for newly added agent {} to client {}", info.id, controller.avatar_id());

            //let mut pathing = pathing.clone();
            //pathing.start_time = time.elapsed_secs();
            //pathing.start_pos = movement.position.into();

            let data = pathing.to_bytes()
                .expect("Failed to serialize Pathing data");

            controller.send_packet(CPktAvatarBehaviors {
                field_1: info.id,
                field_2: data.into(),
                ..Default::default()
            });
        }
    }
}

fn cleanup_targets(
    mut inactive: RemovedComponents<Active>,
    query: Query<Entity, With<NonClientBaseTag>>,
    mut commands: Commands,
) {
    for ent in inactive.read() {
        if query.contains(ent) {
            commands.entity(ent)
                .remove::<NavTarget>()
                .remove::<PathCorridor>()
                .remove::<Pathing>();
        }
    }
}