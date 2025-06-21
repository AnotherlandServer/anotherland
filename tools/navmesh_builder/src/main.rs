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

#![feature(int_roundings)]

use std::{collections::{HashMap, VecDeque}, path::{Path, PathBuf}, str::FromStr, sync::Arc, time::Duration};

use base64::{prelude::BASE64_STANDARD, Engine};
use clap::{Parser, Subcommand};
use console::Term;
use error::{NavMeshBuilderError, NavMeshBuilderResult};
use futures::TryStreamExt;
use glam::{EulerRot, Mat4, Vec3};
use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use once_cell::sync::Lazy;
use pepkg::PePkg;
use plexus::{buffer::{FromRawBuffers, MeshBuffer, MeshBuffer3}, index::{CollectWithIndexer, LruIndexer}, primitive::{cube, decompose::Triangulate, generate::{Generator, Position}, sphere, MapVertices, Trigon}};
use realm_api::{NavmeshBuilder, NavmeshTileBuilder, RealmApi};
use recastnavigation_rs::{detour::{dt_create_nav_mesh_data, DtBuf, DtNavMeshCreateParams}, recast::{rc_build_compact_heightfield, rc_build_contours, rc_build_distance_field, rc_build_poly_mesh, rc_build_poly_mesh_detail, rc_build_regions, rc_calc_bounds, rc_calc_grid_size, rc_create_heightfield, rc_erode_walkable_area, rc_filter_ledge_spans, rc_filter_low_hanging_walkable_obstacles, rc_filter_walkable_low_height_spans, rc_mark_walkable_triangles, rc_rasterize_triangles_1, RcBuildContoursFlags, RcCompactHeightfield, RcConfig, RcContext, RcContourSet, RcHeightfield, RcPolyMesh, RcPolyMeshDetail, RC_WALKABLE_AREA}};
use theon::{adjunct::Map, query::Aabb};
use tokio::{fs, runtime::Handle, sync::Mutex};
use toolkit::{types::Uuid};
use upk::{types::{Level, Model, ObjectProperty, ScriptObject, StaticMesh, StaticMeshCollectionActor, Terrain, TerrainComponent}, Container, ObjectRef};
use log::{debug, error, info, warn};
use url::Url;
use rayon::ThreadPoolBuilder;

mod error;

#[derive(Subcommand)]
enum Commands {
    GenerateMesh {
        #[arg(long, env = "SERVICE_REALM_API_URL", default_value = "http://127.0.0.1:8001")]
        service_realm_url: Url,
        world: Option<String>,
    },
    Export {
        package: String,
        out: String,
    },
}

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    game_folder: String,

    #[command(subcommand)]
    command: Commands,
}

const fn meter_to_uu(meters: f32) -> f32 {
    meters * 100.0
}

const CH: f32 = meter_to_uu(0.075);
const CS: f32 = meter_to_uu(0.1);
const TS: i32 = 2048i32;

#[tokio::main]
async fn main() -> NavMeshBuilderResult<()> {
    env_logger::init();

    let cli = Cli::parse();
    let term = Term::stdout();

    let multiprogress = MultiProgress::with_draw_target(ProgressDrawTarget::term(term, 16));

    match cli.command {
        Commands::GenerateMesh { service_realm_url, world } => {
            let game_path = Path::new(&cli.game_folder);
            let realm = RealmApi::new(service_realm_url);

            let worlds = tokio::task::block_in_place(move || {
                let db = sqlite::open(
                    game_path
                    .join("Atlas/data/otherlandgame/content/dbbba21e-2342-4357-a777-302ed11b978b/instance.db")
                ).unwrap();

                info!("Importing worlddefs...");
            
                let result = db
                    .prepare("SELECT * FROM WorldDef")
                    .unwrap()
                    .into_iter()
                    .map(|row| row.unwrap());

                let mut records = Vec::new();
            
                // dump data
                for row in result {
                    let world_name = row.read::<&str,_>("sWorldDef").to_owned();

                    if 
                        let Some(world) = &world &&
                        &world_name != world 
                    {
                        continue;
                    }

                    records.push((
                        row.read::<i64,_>("ixWorldID") as u16, 
                        row.read::<&str,_>("uxWorldDefGuid").parse::<Uuid>().unwrap(), 
                        row.read::<&str,_>("sWorldDef").to_owned(),
                    ));
                }
            
                records
            });

            for (idx, (id, guid, level)) in worlds.iter().enumerate() {
                let _ = multiprogress.println(format!("[{}/{}] Building mesh for level: {level}", idx + 1, worlds.len()));

                if !game_path.join("UnrealEngine3/AmunGame/CookedPCConsole").join(format!("{level}.upk")).exists() {
                    warn!("Skipping level {level}. No level data found!");
                    continue;
                }

                let mut pepkg = PePkg::open(
                    game_path.join(format!("Atlas/data/otherlandgame/WorldData/{level}/{level}.pepkg"))
                )?;

                let federation_mesh = pepkg.read_federation_file()?;

                let world_mesh = build_level_mesh(&multiprogress, &cli.game_folder, level).await?;

                let verts = world_mesh.as_vertex_slice()
                    .iter()
                    .map(|v| [v.x, v.y, v.z])
                    .collect::<Vec<_>>();

                let (min_bounds, max_bounds) = rc_calc_bounds(&verts);
                let (grid_width, grid_height) = rc_calc_grid_size(&min_bounds, &max_bounds, CS);
                let (width, height) = rc_calc_grid_size(&min_bounds, &max_bounds, CS);

                drop(verts);

                let tw = grid_width.div_ceil(TS);
                let th = grid_height.div_ceil(TS);
                let tcs = TS as f32 * CS;

                let tile_count = tw * th;

                let mut res = realm.query_navmeshs()
                    .world_guid(*guid)
                    .query()
                    .await?;

                let db_navmesh = if let Some(db_navmesh) = res.try_next().await? {
                    db_navmesh
                } else {
                    realm.create_navmesh(
                        NavmeshBuilder::default()
                            .id(Uuid::new())
                            .world_id(*id as i32)
                            .world_guid(*guid)
                            .origin([min_bounds[0] as f64, min_bounds[1] as f64, min_bounds[2] as f64])
                            .tile_width(tcs.into())
                            .tile_height(tcs.into())
                            .pathengine_start_x(federation_mesh.start_x)
                            .pathengine_start_y(federation_mesh.start_y)
                            .pathengine_tile_size(federation_mesh.tile_size)
                            .pathengine_tile_pitch(federation_mesh.width.div_ceil(federation_mesh.tile_size))
                            .build()
                            .unwrap()
                    ).await?
                };

                let thread_pool = ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .expect("Failed to build thread pool");

                let world_count = worlds.len();
                let world_mesh = Arc::new(world_mesh);
                let handle = Handle::current();

                thread_pool.scope(|s| {
                    (0..th)
                        .flat_map(|y| {
                            (0..tw).map(move |x| (x, y))
                        })
                        .enumerate()
                        .for_each(|(current_tile, (x, y))| {
                            let multiprogress = multiprogress.clone();
                            let world_mesh = world_mesh.clone();
                            let realm = realm.clone();
                            let handle = handle.clone();
                            
                            s.spawn(move |_| {
                                let tile_bounds = Aabb {
                                    origin: Vec3::new(
                                        min_bounds[0] + x as f32 * tcs,
                                        min_bounds[1],
                                        min_bounds[2] + y as f32 * tcs,
                                    ),
                                    extent: Vec3::new(
                                        tcs,
                                        max_bounds[1] - min_bounds[1],
                                        tcs,
                                    )
                                };

                                let config = RcConfig {
                                    ch: CH,
                                    cs: CS,
                                    walkable_height: (meter_to_uu(2.0) / CH).ceil() as i32,
                                    walkable_radius: (meter_to_uu(1.0) / CS).ceil() as i32,
                                    walkable_climb: (meter_to_uu(0.3) / CH).ceil() as i32,
                                    walkable_slope_angle: 45.0,
                                    width: grid_width,
                                    height: grid_height,
                                    border_size: (meter_to_uu(0.1) / CS).ceil() as i32 * 3,
                                    ..Default::default()
                                };

                                let progress = multiprogress.add(ProgressBar::new_spinner());
                                progress.set_style(ProgressStyle::with_template(&format!(
                                    "[{}/{}][{}/{}]  {{msg}}", 
                                    idx + 1, world_count,
                                    current_tile + 1, tile_count
                                )).unwrap());
                                progress.set_message("Waiting...");

                                let tile_exists = {
                                    let realm = realm.clone();
                                    let _guard = handle.enter();
                                    handle.block_on(async move {
                                        let mut res = realm.query_navmesh_tiles()
                                            .mesh_id(db_navmesh.id)
                                            .tile_x(x)
                                            .tile_y(y)
                                            .query()
                                            .await?;

                                        if res.try_next().await?.is_some() {
                                            Ok::<_, NavMeshBuilderError>(false)
                                        } else {
                                            Ok(false)
                                        }
                                    })
                                }.unwrap();

                                if tile_exists {
                                    progress.finish_with_message("Tile already exists, skipping...");
                                    return;
                                }

                                let buf = build_tile(
                                    multiprogress.clone(),
                                    progress.clone(), 
                                    &config, 
                                    &world_mesh, 
                                    tile_bounds,
                                    x,
                                    y,
                                ).expect("Failed to build tile");

                                if let Some(res) = buf {
                                    let buf = res.as_slice().to_vec();
                                    drop(res);

                                    let _guard = handle.enter();
                                    handle.block_on(async move {
                                        realm.create_navmesh_tile(
                                            NavmeshTileBuilder::default()
                                                .id(Uuid::new())
                                                .mesh_id(db_navmesh.id)
                                                .tile_x(x)
                                                .tile_y(y)
                                                .data(BASE64_STANDARD.encode(&buf))
                                                .build()
                                                .unwrap()
                                        ).await
                                        .expect("Failed to store navmesh tile");

                                        progress.finish_with_message(format!("{} bytes", buf.len()));
                                    });
                                } else {
                                    progress.finish_with_message("Empty!");
                                }
                            });
                        });
                });
            }

            let _ = multiprogress.println("All done!");
        },
        Commands::Export { package, out } => {
            let _ = multiprogress.println(format!("Building mesh for level: {package}"));
            let world_mesh = build_level_mesh(&multiprogress, &cli.game_folder, &package).await?;

            // Save the world_mesh as an OBJ file
            write_obj(&package, &world_mesh, out).await?;
        },
    }

    Ok(())
}

fn extract_poligons(mesh: &MeshBuffer3<u64, Vec3>, bounds: &Aabb<Vec3>) -> NavMeshBuilderResult<MeshBuffer3<u32, Vec3>> {
    let min = bounds.lower_bound();
    let max = bounds.upper_bound();

    let indices = mesh.as_index_slice();
    let vertices = mesh.as_vertex_slice();

    Ok(
        indices.iter()
            .map(|polygon| {
                polygon.map(|i| {
                    vertices[i as usize]
                })
            })
            .filter(|poly| {
                // Check if any vertex of the polygon is within bounds
                poly.0.iter().any(|v| {
                    v.x >= min.x && v.x <= max.x &&
                    v.y >= min.y && v.y <= max.y &&
                    v.z >= min.z && v.z <= max.z
                })
            })
            .collect_with_indexer(LruIndexer::default())?
    )
}

fn build_tile(_multiprogress: MultiProgress, progress: ProgressBar, config: &RcConfig, mesh: &MeshBuffer3<u64, Vec3>, mut bounds: Aabb<Vec3>, tx: i32, ty: i32) -> NavMeshBuilderResult<Option<DtBuf>> {
    progress.set_message("Rasterizing heightfield...");

    let mut rc = RcContext::new(true);

    bounds.origin -= Vec3::new(config.border_size as f32 * CS, 0.0, config.border_size as f32 * CS);
    bounds.extent += Vec3::new(config.border_size as f32 * CS * 2.0, 0.0, config.border_size as f32 * CS * 2.0);

    let mesh = extract_poligons(mesh, &bounds)?;

    if mesh.as_index_slice().is_empty() {
        return Ok(None);
    }

    let min_bounds = [bounds.lower_bound().x, bounds.lower_bound().y, bounds.lower_bound().z];
    let max_bounds = [bounds.upper_bound().x, bounds.upper_bound().y, bounds.upper_bound().z];

    let tris = mesh.as_index_slice()
        .iter()
        .map(|&i| {
            [
                i.0[2] as i32,
                i.0[1] as i32,
                i.0[0] as i32
            ]
        })
        .collect::<Vec<_>>();

    let verts = mesh.as_vertex_slice()
        .iter()
        .map(|v| [v.x, v.y, v.z])
        .collect::<Vec<_>>();

    let mut tri_area_ids = vec![0u8; tris.len()];

    rc_mark_walkable_triangles(
        &mut rc, 
        config.walkable_slope_angle, 
        &verts, 
        &tris, 
        &mut tri_area_ids
    )?;

    let mut heightfield = RcHeightfield::new();

    rc_create_heightfield(
        &mut rc, 
        &mut heightfield, 
        config.width, 
        config.height, 
        &min_bounds, 
        &max_bounds, 
        CS, 
        CH
    )?;

    rc_rasterize_triangles_1(
        &mut rc, 
        &verts,
        &tris,
        &tri_area_ids,
        &mut heightfield,
        config.walkable_climb
    )?;

    progress.set_message("Cleaning up heightfield...");
    rc_filter_low_hanging_walkable_obstacles(
        &mut rc, 
        config.walkable_climb, 
        &mut heightfield
    );

    rc_filter_ledge_spans(
        &mut rc, 
        config.walkable_height, 
        config.walkable_climb, 
        &mut heightfield
    );

    rc_filter_walkable_low_height_spans(
        &mut rc, 
        config.walkable_height, 
        &mut heightfield
    );

    progress.set_message("Building compact height field...");

    let mut compact_height_field = RcCompactHeightfield::new();
    rc_build_compact_heightfield(
        &mut rc,
        config.walkable_height,
        config.walkable_climb,
        &heightfield,
        &mut compact_height_field
    )?;

    drop(heightfield);

    rc_erode_walkable_area(
        &mut rc, 
        config.walkable_radius, 
        &mut compact_height_field
    )?;

    progress.set_message("Building distance field...");
    rc_build_distance_field(&mut rc, &mut compact_height_field)?;

    progress.set_message("Building regions field...");
    rc_build_regions(
        &mut rc, 
        &mut compact_height_field, 
        (meter_to_uu(0.01) / CS).ceil() as i32, 
        4, 
        20
    )?;

    progress.set_message("Building contours...");

    let mut contours = RcContourSet::new();
    rc_build_contours(
        &mut rc, 
        &compact_height_field, 
        1.3, 
        0, 
        &mut contours, 
        RcBuildContoursFlags::RC_CONTOUR_TESS_WALL_EDGES
    )?;

    progress.set_message("Building poly meshes...");

    let mut mesh = RcPolyMesh::new();
    rc_build_poly_mesh(
        &mut rc,
        &contours,
        6,
        &mut mesh,
    )?;

    if mesh.npolys() == 0 {
        return Ok(None);
    }

    progress.set_message("Building detail poly meshes...");


    let mut detail_mesh = RcPolyMeshDetail::new();
    rc_build_poly_mesh_detail(
        &mut rc, 
        &mesh, 
        &compact_height_field,
        meter_to_uu(6.0), 
        meter_to_uu(1.0), 
        &mut detail_mesh
    )?;

    // Update polygon flags
    for i in 0..mesh.npolys() {
        if mesh.areas()[i] == RC_WALKABLE_AREA {
            mesh.flags_mut()[i] = 1;
        } else {
            mesh.flags_mut()[i] = 0;
        }
    }

    progress.set_message("Building navmesh...");
    let mut params = DtNavMeshCreateParams {
        // Polygon mesh attributes
        verts: Some(mesh.verts()),
        polys: Some(mesh.polys()),
        poly_flags: Some(mesh.flags()),
        poly_areas: Some(mesh.areas()),
        nvp: 6,

        // Height detail attributes
        detail_meshes: Some(detail_mesh.meshes()),
        detail_verts: Some(detail_mesh.verts()),
        detail_tris: Some(detail_mesh.tris()),

        // General configuration
        walkable_climb: config.walkable_climb as f32,
        walkable_height: config.walkable_height as f32,
        walkable_radius: config.walkable_radius as f32,
        cs: CS,
        ch: CH,
        build_bv_tree: true,

        // Tile configuration
        tile_x: tx,
        tile_y: ty,
        bmin: [min_bounds[0], min_bounds[1], min_bounds[2]],
        bmax: [max_bounds[0], max_bounds[1], max_bounds[2]],
        
        ..Default::default()
    };

    let buf = dt_create_nav_mesh_data(&mut params)?;

    Ok(Some(buf))
}

async fn build_level_mesh(mp: &MultiProgress, game_path: &str, level: &str) -> NavMeshBuilderResult<MeshBuffer3<u64, Vec3>> {
    let mut level_queue: VecDeque<String> = VecDeque::from(vec![
        level.to_string(),
    ]);

    let mut world_mesh = MeshBuffer3::<u64, Vec3>::new();

    while let Some(package) = level_queue.pop_front() {
        clear_meshes().await;

        let progress = mp.add(ProgressBar::new_spinner());
        progress.set_style(ProgressStyle::with_template("  {msg} {spinner} ").unwrap());
        progress.enable_steady_tick(Duration::from_millis(100));
        progress.set_message(format!("Loading level: {package}"));

        let mut container = Container::new(PathBuf::from_str(game_path)
            .expect("invalid path")
            .join("UnrealEngine3/AmunGame/CookedPCConsole")
        );

        container.mount_package("Atlas").await
            .expect("failed to mount package");

        container.mount_package("Otherland").await
            .expect("failed to mount package");

        container.mount_package("PathEngine").await
            .expect("failed to mount package");

        container.mount_package("UI_GFx").await
            .expect("failed to mount package");

        container.mount_package("Startup").await
            .expect("failed to mount package");

        if container.mount_package(&package).await.is_err() {
            error!("Failed to mount package: {package}");
            continue;
        }

        if let Some(world) = container.lookup_object("World:TheWorld") {
            for child in world.children() {
                match child.class().name() {
                    "LevelStreamingNeighbor" | "LevelStreamingKismet" => {
                        let script_obj = container.deserialize::<ScriptObject>(child).await
                            .expect("failed to deserialize LevelStreamingNeighbor");

                        let package_name = script_obj.attrib("PackageName")
                            .and_then(|v| if let ObjectProperty::Name(s) = v { Some(s.clone()) } else { None })
                            .expect("Invalid PackageName");

                        debug!("Found LevelStreamingNeighbor: {package_name}");

                        // Add the level to the queue for processing
                        level_queue.push_back(package_name.to_string());
                    },
                    _ => {},
                }
            }
        } else {
            error!("TheWorld not found in package: {package}");
            continue;
        }

        progress.disable_steady_tick();
        progress.finish();

        if let Some(level) = container.lookup_object("Level:TheWorld/PersistentLevel") {
            let (_scrip_obj, ulevel) = container.deserialize::<(ScriptObject, Level)>(&level).await
                .expect("failed to deserialize PersistentLevel");

            debug!("Loaded PersistentLevel: {ulevel:#?}");

            let (_scrip_obj, model) = container.deserialize::<(ScriptObject, Model)>(&ulevel.model).await
                .expect("failed to deserialize Model");

            debug!("Loaded Model: {model:#?}");

            let progress = mp.add(ProgressBar::new(ulevel.actors.len() as u64));
            progress.set_style(ProgressStyle::with_template("  {wide_msg}: {bar:40.magenta/red} {pos:>7}/{len:7}").unwrap());
            progress.set_message(format!("Processing level: {package}"));
            
            // Walk trough all actors
            for actor in ulevel.actors {
                debug!("Processing actor: {} - {}", actor.class().name(), actor.fully_qualified_name());

                progress.inc(1);

                match actor.class().name() {
                    "Terrain" => {
                        let (script_obj, terrain) = container.deserialize::<(ScriptObject, Terrain)>(&actor).await
                            .expect("failed to deserialize Terrain");

                        let num_patches_x = script_obj.attrib("NumPatchesX")
                            .and_then(|v| if let ObjectProperty::Int(i) = v { Some(*i) } else { None })
                            .unwrap() as usize;

                        let num_patches_y = script_obj.attrib("NumPatchesY")
                            .and_then(|v| if let ObjectProperty::Int(i) = v { Some(*i) } else { None })
                            .unwrap() as usize;

                        let num_vertices_x = script_obj.attrib("NumVerticesX")
                            .and_then(|v| if let ObjectProperty::Int(i) = v { Some(*i) } else { None })
                            .unwrap() as usize;

                        let num_vertices_y = script_obj.attrib("NumVerticesY")
                            .and_then(|v| if let ObjectProperty::Int(i) = v { Some(*i) } else { None })
                            .unwrap() as usize;

                        let draw_scale = script_obj.attrib("DrawScale3D")
                            .and_then(|v| if let ObjectProperty::Vector(vec) = v { Some(Vec3::from_slice(vec)) } else { None })
                            .unwrap_or(Vec3::ONE);

                        let generate_navigation_mesh = script_obj.attrib("bGenerateNavigationMesh")
                            .and_then(|v| if let ObjectProperty::Bool(b) = v { Some(*b) } else { None })
                            .unwrap_or(true);

                        if !generate_navigation_mesh {
                            debug!("Skipping Terrain actor: {} - bGenerateNavigationMesh is false", actor.fully_qualified_name());
                            continue;
                        }

                        let terrain_components = script_obj.attrib("TerrainComponents")
                            .and_then(|v| if let ObjectProperty::Array(arr) = v { Some(arr) } else { None })
                            .unwrap()
                            .iter()
                            .filter_map(|v| {
                                if let ObjectProperty::Component(obj) = v {
                                    Some(obj)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>();

                        debug!("Terrain patch count {num_patches_x} x {num_patches_y}");
                        debug!("Num Vertices: {num_vertices_x} x {num_vertices_y}");
                        debug!("Draw Scale: {draw_scale}");

                        debug!("Heights: {}", terrain.heights.len());
                        debug!("Info Data: {}", terrain.info_data.len());

                        let mut terrain_mesh = MeshBuffer::<Trigon<u64>, Vec3>::new();

                        for component in terrain_components {
                            let (component_obj, component) = container.deserialize::<(ScriptObject, TerrainComponent)>(component).await
                                .expect("failed to deserialize Terrain");

                            let true_section_size_x = component_obj.attrib("TrueSectionSizeX")
                                .and_then(|v| if let ObjectProperty::Int(i) = v { Some(*i) } else { None })
                                .unwrap_or(0);

                            let true_section_size_y = component_obj.attrib("TrueSectionSizeY")
                                .and_then(|v| if let ObjectProperty::Int(i) = v { Some(*i) } else { None })
                                .unwrap_or(0);

                            debug!("Section Size: {true_section_size_x} x {true_section_size_y}");
                            debug!("Collision verts: {}", component.collision_vertices.len());

                            let mut section_mesh = MeshBuffer::<Trigon<u64>, Vec3>::from_raw_buffers(
                                (0..true_section_size_y).flat_map(|y| {
                                    (0..true_section_size_x).flat_map(move |x| {
                                        let index = (y * (true_section_size_x + 1) + x) as u64;
                                        vec![
                                            Trigon::new(
                                                index, 
                                                index + true_section_size_x as u64 + 1, 
                                                index + 1
                                            ),
                                            Trigon::new(
                                                index + 1, 
                                                index + true_section_size_x as u64 + 1, 
                                                index + true_section_size_x as u64 + 2
                                            )
                                        ]
                                    })
                                }).collect::<Vec<_>>(),
                                component.collision_vertices
                            )?;

                            terrain_mesh.append(&mut section_mesh)?;
                        }

                        world_mesh.append(&mut terrain_mesh)?;
                    },
                    "StaticMeshActor" | "InterpActor" => {
                        let actor_obj = container.deserialize::<ScriptObject>(&actor).await
                            .expect("failed to deserialize StaticMeshActor");

                        let static_mesh_component_ref = actor_obj.attrib("CollisionComponent")
                            .or(actor_obj.attrib("StaticMeshComponent"))
                            .and_then(|v| if let ObjectProperty::Object(r) = v { Some(r) } else { None })
                            .expect("Invalid StaticMeshComponent");

                        let static_mesh_component = get_cached_static_mesh_component(&container, static_mesh_component_ref).await?;

                        if 
                            static_mesh_component.is_relevant_for_navigation() &&
                            let Some(static_mesh) = &static_mesh_component.collision_mesh
                        {
                            debug!("Processing StaticMeshActor: {} - Mesh {}", 
                                actor.fully_qualified_name(), 
                                static_mesh_component.static_mesh_obj.as_ref().unwrap().fully_qualified_name()
                            );

                            let location = 
                                Mat4::from_translation(
                                    actor_obj.attrib("Location")
                                        .and_then(|v| if let ObjectProperty::Vector(v) = v { Some(*v) } else { None })
                                        .map(|v| v.into())
                                        .unwrap_or(Vec3::ZERO)
                                );

                            let draw_scale_3d = 
                                Mat4::from_scale(
                                    actor_obj.attrib("DrawScale3D")
                                        .and_then(|v| if let ObjectProperty::Vector(v) = v { Some((*v).into()) } else { None })

                                        .unwrap_or(Vec3::ONE) * static_mesh_component.scale_3d

                                );

                            let rotation = actor_obj.attrib("Rotation")
                                    .and_then(|v| 
                                        if let ObjectProperty::Rotator(v) = v { 
                                            Some(Mat4::from_euler(
                                                EulerRot::YXZ,
                                                v[0] * (180.0 / 32768.0), 
                                                v[1] * (180.0 / 32768.0), 
                                                v[2] * (180.0 / 32768.0)
                                            )) 
                                        } else { 
                                            None 
                                        })
                                    .unwrap_or(Mat4::IDENTITY);

                            let transform = location * rotation * draw_scale_3d;

                            let instanced = MeshBuffer::<Trigon<u64>, Vec3>::from_raw_buffers(
                                static_mesh.as_index_slice().to_vec(), 
                                static_mesh.as_vertex_slice().to_vec()
                            )?;

                            let mut instanced = instanced
                                .map_vertices(|v| {
                                    transform.transform_point3(v)
                                });

                            world_mesh.append(&mut instanced)?;
                        } else {
                            debug!("Skipping StaticMeshActor: {}", actor.fully_qualified_name(),);
                        }
                    },
                    "StaticMeshCollectionActor" => {
                        let (actor_obj, actor_data) = container.deserialize::<(ScriptObject, StaticMeshCollectionActor)>(&actor).await
                            .expect("failed to deserialize StaticMeshCollectionActor");

                        let components = actor_obj.attrib("StaticMeshComponents")
                            .and_then(|v| if let ObjectProperty::Array(arr) = v { Some(arr) } else { None })
                            .unwrap()
                            .iter()
                            .zip(actor_data.parent_to_world.iter())
                            .filter_map(|(v, mat)| {
                                if let ObjectProperty::Component(obj) = v {
                                    Some((obj, mat))
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>();

                        for &(component, transform) in components.iter() {
                            let static_mesh_component = get_cached_static_mesh_component(&container, component).await
                                .expect("failed to deserialize StaticMeshComponent");

                            if 
                                static_mesh_component.is_relevant_for_navigation() &&
                                let Some(static_mesh) = &static_mesh_component.collision_mesh
                            {
                                debug!("Processing StaticMeshComponent: {} - Mesh {}", 
                                    component.fully_qualified_name(), 
                                    static_mesh_component.static_mesh_obj.as_ref().unwrap().fully_qualified_name()
                                );

                                let instanced = MeshBuffer::<Trigon<u64>, Vec3>::from_raw_buffers(
                                    static_mesh.as_index_slice().to_vec(), 
                                    static_mesh.as_vertex_slice().to_vec()
                                ).unwrap();

                                let transform = *transform * Mat4::from_scale(static_mesh_component.scale_3d);

                                let mut instanced = instanced
                                    .map_vertices(|v| {
                                        transform.transform_point3(v)
                                    });

                                world_mesh.append(&mut instanced).expect("Failed to append instanced mesh");
                            } else {
                                debug!("Skipping StaticMeshComponent: {}", component.fully_qualified_name());
                            }
                        }
                    },
                    "BlockingVolume" => {
                        let script_obj = container.deserialize::<ScriptObject>(&actor).await
                            .expect("failed to deserialize BlockingVolume");

                        let brush_component_obj = script_obj.attrib("BrushComponent")
                            .and_then(|v| if let ObjectProperty::Object(r) = v { Some(r) } else { None })
                            .expect("Invalid BrushComponent");

                        let brush_component = container.deserialize::<ScriptObject>(brush_component_obj).await
                            .expect("failed to deserialize Brush Model");

                        debug!("Processing BlockingVolume: {} - Component {}", actor.fully_qualified_name(), brush_component_obj.fully_qualified_name());

                        if let Some(mesh) = brush_component.attrib("BrushAggGeom")
                            .and_then(|v| if let ObjectProperty::Struct(_, geom) = v { Some(geom) } else { None })
                            .map(mesh_from_agg_geom) 
                        {
                            let transform = Mat4::from_translation(
                                script_obj.attrib("Location")
                                    .and_then(|v| if let ObjectProperty::Vector(v) = v { Some(*v) } else { None })
                                    .map(|v| v.into())
                                    .unwrap_or(Vec3::ZERO)
                            );

                            let mut mesh = mesh
                                .map_vertices(|v| {
                                    transform.transform_point3(v)
                                });

                            world_mesh.append(&mut mesh).expect("Failed to append BlockingVolume mesh");
                        }
                    },
                    _ => {},
                }
            }
        
            progress.finish();
        }
    }

    // Convert scale
    world_mesh = world_mesh
        .map_vertices(|Vec3 { x, y, z }| Vec3::new(y, z, x));

    Ok(world_mesh)
}

pub struct StaticMeshComponent {
    pub static_mesh_unreal: Option<Arc<(ScriptObject, StaticMesh)>>,
    pub static_mesh_obj: Option<ObjectRef>,
    pub static_mesh: Option<MeshBuffer3<u64, Vec3>>,
    pub collision_mesh: Option<MeshBuffer3<u64, Vec3>>,
    pub scale_3d: Vec3,
    pub component: ScriptObject,
}

impl StaticMeshComponent {
    pub fn is_relevant_for_navigation(&self) -> bool {
        if 
            let Some((_, static_mesh)) = self.static_mesh_unreal.as_deref() && 
            static_mesh.body_setup.is_none()
        {
            return false;
        }

        let collide_actors = self.component.attrib("CollideActors")
            .and_then(|v| if let ObjectProperty::Bool(b) = v { Some(*b) } else { None })
            .unwrap_or(true);

        let block_rigid_body = self.component.attrib("BlockRigidBody")
            .and_then(|v| if let ObjectProperty::Bool(b) = v { Some(*b) } else { None })
            .unwrap_or(false);

        let generate_navigation_mesh = self.component.attrib("bGenerateNavigationMesh")
            .and_then(|v| if let ObjectProperty::Bool(b) = v { Some(*b) } else { None })
            .unwrap_or(false);

        let collision_channels = self.component.attrib("RBCollideWithChannels")
            .and_then(|v| {
                debug!("RBCollideWithChannels: {v:#?}");
                if let ObjectProperty::Struct(_, channels) = v { Some(channels) } else { None }
            })
            .map(|channels| {
                channels.attrib("Pawn")
                    .and_then(|v| if let ObjectProperty::Bool(b) = v { Some(*b) } else { None })
                    .unwrap_or(false) ||
                channels.attrib("GameplayPhysics")
                    .and_then(|v| if let ObjectProperty::Bool(b) = v { Some(*b) } else { None })
                    .unwrap_or(false)
            })
            .unwrap_or(false);

        collide_actors || block_rigid_body || generate_navigation_mesh || collision_channels
    }
}

#[allow(clippy::type_complexity)]
static STATIC_MESHES : Lazy<Mutex<HashMap<String, Arc<(ScriptObject, StaticMesh)>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

static STATIC_MESH_COMPONENTS: Lazy<Mutex<HashMap<String, Arc<StaticMeshComponent>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

async fn clear_meshes() {
    STATIC_MESHES.lock().await.clear();
    STATIC_MESH_COMPONENTS.lock().await.clear();
}

fn mesh_from_agg_geom(geom: &ScriptObject) -> MeshBuffer3<u64, Vec3> {
    let mut mesh = MeshBuffer::<Trigon<u64>, Vec3>::new();

    geom.attrib("SphereElems")
        .and_then(|v| if let ObjectProperty::Array(arr) = v { Some(arr) } else { None })
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| {
            if let ObjectProperty::Struct(_, sphere) = v {
                Some(sphere)
            } else {
                None
            }
        })
        .for_each(|sphere| {
            let radius = sphere.attrib("Radius")
                .and_then(|v| if let ObjectProperty::Float(f) = v { Some(*f) } else { None })
                .unwrap_or(0.0);

            let tm = sphere.attrib("TM")
                .and_then(|v| if let ObjectProperty::Matrix(v) = v { Some(*v) } else { None })
                .unwrap_or(Mat4::IDENTITY)
                * Mat4::from_scale(Vec3::new(radius * 2.0, radius * 2.0, radius * 2.0));

            let mut sphere_mesh = sphere::UvSphere::new(16, 8)
                .polygons::<Position<Vec3>>()
                .triangulate()
                .map_vertices(|v| tm.transform_point3(v))
                .collect_with_indexer::<MeshBuffer3<u64, Vec3>, _>(LruIndexer::default())
                .expect("Failed to create sphere mesh");
            
            mesh.append(&mut sphere_mesh).expect("Failed to append sphere mesh");
        });

    geom.attrib("BoxElems")
        .and_then(|v| if let ObjectProperty::Array(arr) = v { Some(arr) } else { None })
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| {
            if let ObjectProperty::Struct(_, cube) = v {
                Some(cube)
            } else {
                None
            }
        })
        .for_each(|cube| {
            let x = cube.attrib("X")
                .and_then(|v| if let ObjectProperty::Float(f) = v { Some(*f) } else { None })
                .unwrap_or(0.0);

            let y = cube.attrib("Y")
                .and_then(|v| if let ObjectProperty::Float(f) = v { Some(*f) } else { None })
                .unwrap_or(0.0);

            let z = cube.attrib("Z")
                .and_then(|v| if let ObjectProperty::Float(f) = v { Some(*f) } else { None })
                .unwrap_or(0.0);

            let tm = cube.attrib("TM")
                .and_then(|v| if let ObjectProperty::Matrix(v) = v { Some(*v) } else { None })
                .unwrap_or(Mat4::IDENTITY) *
                Mat4::from_scale(Vec3::new(x, y, z));

            let mut cube_mesh = cube::Cube::new()
                .polygons::<Position<Vec3>>()
                .triangulate()
                .map_vertices(|v| tm.transform_point3(v))
                .collect_with_indexer::<MeshBuffer3<u64, Vec3>, _>(LruIndexer::default())
                .expect("Failed to create box mesh");
            
            mesh.append(&mut cube_mesh).expect("Failed to append box mesh");
        });

    geom.attrib("SphylElems")
        .and_then(|v| if let ObjectProperty::Array(arr) = v { Some(arr) } else { None })
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| {
            if let ObjectProperty::Struct(_, cube) = v {
                Some(cube)
            } else {
                None
            }
        })
        .for_each(|_sphyl| {
            todo!(); // SphylElems are not yet implemented
        });

    geom.attrib("ConvexElems")
        .and_then(|v| if let ObjectProperty::Array(arr) = v { Some(arr) } else { None })
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| {
            if let ObjectProperty::Struct(_, convex) = v {
                Some(convex)
            } else {
                None
            }
        })
        .for_each(|convex| {
            let vertices = convex.attrib("VertexData")
                .and_then(|v| if let ObjectProperty::Array(arr) = v { Some(arr) } else { None })
                .unwrap()
                .iter()
                .filter_map(|v| {
                    if let ObjectProperty::Vector(v) = v {
                        Some(Vec3::from_slice(v))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let indices = convex.attrib("FaceTriData")
                .and_then(|v| if let ObjectProperty::Array(arr) = v { Some(arr) } else { None })
                .unwrap()
                .iter()
                .filter_map(|v| {
                    if let ObjectProperty::Int(i) = v {
                        Some(*i as u64)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if vertices.len() < 3 || indices.len() < 3 {
                return;
            }

            let mut convex_mesh = MeshBuffer::<Trigon<u64>, Vec3>::from_raw_buffers(
                indices.chunks(3).map(|chunk| Trigon::new(chunk[0], chunk[1], chunk[2])),
                vertices
            ).expect("Failed to create convex mesh");

            mesh.append(&mut convex_mesh).expect("Failed to append convex mesh");
        });

    mesh
}

async fn get_cached_static_mesh_component(
    container: &Container,
    object_ref: &ObjectRef,
) -> NavMeshBuilderResult<Arc<StaticMeshComponent>> {
    if let Some(component) = STATIC_MESH_COMPONENTS.lock().await.get(&object_ref.fully_qualified_name().to_string()) {
        return Ok(component.clone());
    }

    let component = container.deserialize::<ScriptObject>(object_ref).await?;

    let scale_3d = component.attrib("Scale3D")
        .and_then(|v| if let ObjectProperty::Vector(f) = v { Some(Vec3::from_array(*f)) } else { None })
        .unwrap_or(Vec3::ONE);


    let mesh_ref = component.attrib("StaticMesh")
        .and_then(|v| if let ObjectProperty::Object(r) = v { Some(r) } else { None });

    let static_mesh_unreal = if let Some(mesh_ref) = mesh_ref {
        Some(
            get_cached_static_mesh(container, mesh_ref).await?
                .ok_or(NavMeshBuilderError::Other(anyhow::anyhow!("StaticMesh not found")))?
        )
    } else {
        None
    };

    let static_mesh = 
        if 
            let Some((_, unreal_mesh)) = static_mesh_unreal.as_deref() &&
            let Some(primary_mesh) = unreal_mesh.lod_meshes.first()
        {
            let mesh = MeshBuffer::<Trigon<u64>, Vec3>::from_raw_buffers(
                primary_mesh.index_buffer.chunks(3)
                    .map(|chunk| {
                        Trigon::new(chunk[0] as u64, chunk[1] as u64, chunk[2] as u64)
                    }), 
                primary_mesh.position_vertex_buffer.data.clone(),
            )?;

            Some(mesh)
        } else {
            None
        };

    let collision_mesh = 
        if 
            let Some((_, unreal_mesh)) = static_mesh_unreal.as_deref() &&
            let Some(collision) = &unreal_mesh.body_setup
        {
            let body_setup = container.deserialize::<ScriptObject>(collision).await
                .expect("Failed to deserialize BodySetup");

            body_setup.attrib("AggGeom")
                .and_then(|v| if let ObjectProperty::Struct(_, geom) = v { Some(geom) } else { None })
                .map(mesh_from_agg_geom)
        } else {
            None
        };

    let static_mesh_component = Arc::new(StaticMeshComponent {
        static_mesh_unreal,
        static_mesh_obj: mesh_ref.cloned(),
        static_mesh,
        collision_mesh,
        scale_3d,
        component,
    });

    STATIC_MESH_COMPONENTS.lock().await
        .insert(object_ref.fully_qualified_name().to_string(), static_mesh_component.clone());

    Ok(static_mesh_component)
}

async fn get_cached_static_mesh(
    container: &Container,
    mesh_ref: &ObjectRef,
) -> NavMeshBuilderResult<Option<Arc<(ScriptObject, StaticMesh)>>> {
    if let Some(mesh) = STATIC_MESHES.lock().await.get(&mesh_ref.fully_qualified_name().to_string()) {
        return Ok(Some(mesh.clone()));
    }

    let (obj, mesh) = container.deserialize::<(ScriptObject, StaticMesh)>(mesh_ref).await?;
    let mesh = Arc::new((obj, mesh));

    STATIC_MESHES.lock().await
        .insert(mesh_ref.fully_qualified_name().to_string(), mesh.clone());

    Ok(Some(mesh))
}

async fn write_obj(
    name: &str,
    mesh: &MeshBuffer3<u64, Vec3>,
    path: impl AsRef<Path>,
) -> NavMeshBuilderResult<()> {
    let mut buffer = String::new();

    // Write header comment
    buffer.push_str("# Generated by navmesh_builder\n");
    buffer.push_str(&format!("# Object: {name}\n\n"));
    
    // Write vertices
    for vertex in mesh.as_vertex_slice() {
        buffer.push_str(&format!("v {} {} {}\n", vertex.x, vertex.y, vertex.z));
    }
    
    buffer.push('\n');
    
    // Write faces (OBJ uses 1-based indexing)
    for trigon in mesh.as_index_slice() {
        buffer.push_str(&format!("f {} {} {}\n", 
            trigon.0[0] as usize + 1,
            trigon.0[1] as usize + 1,
            trigon.0[2] as usize + 1
        ));
    }

    fs::write(path, buffer.as_bytes()).await
        .map_err(NavMeshBuilderError::IoError)?;

    Ok(())
}
