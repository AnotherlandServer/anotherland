use std::{collections::{HashMap, HashSet}, fs, ops::{Deref, DerefMut}};

use bevy_full::{asset::RenderAssetUsages, color::palettes::css::WHITE, input::mouse::MouseMotion, log::tracing_subscriber::field::debug, pbr::wireframe::{WireframeConfig, WireframePlugin}, prelude::*, render::{mesh::{Indices, PrimitiveTopology}, settings::{WgpuFeatures, WgpuSettings}, RenderPlugin}, window::CursorGrabMode};
use clap::{Parser, Subcommand};
use log::{error, info};
use lyon::{geom::point, path::{builder::NoAttributes, traits::PathBuilder, BuilderImpl, Path}, tessellation::{BuffersBuilder, FillOptions, FillTessellator, FillVertex, StrokeOptions, StrokeTessellator, StrokeVertex, VertexBuffers}};
use pepkg::PePkg;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Extract {
        pepkg_path: String,
        output_path: String,
    },
    Display {
        pepkg_path: String,
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Extract { pepkg_path, output_path }) => {
            env_logger::builder()
                .filter_level(if cli.verbose {
                    log::LevelFilter::Trace
                } else {
                    log::LevelFilter::Info
                })
                .init();

            info!("Extracting {pepkg_path} to {output_path}");

            if std::path::Path::new(&pepkg_path).exists() {
                let mut pepkg = PePkg::open(&pepkg_path).expect("Failed to open pepkg file");
                let output_path = std::path::Path::new(&output_path);
                if !output_path.exists() {
                    std::fs::create_dir_all(output_path).expect("Failed to create output directory");
                }
                
                let federation = pepkg.read_federation_file_as_xml().unwrap();
                fs::write(output_path.join("federation.xml"), federation.as_bytes()).unwrap();

                for i in 0 .. pepkg.tile_count() {
                    let file = pepkg.read_tile_as_xml(i).unwrap();

                    for (idx, entry) in file.iter().enumerate() {

                        let file_name = match idx {
                            0 => format!("tile_{i}_mesh.xml"),
                            1 => format!("tile_{i}_collisionPreprocess.xml"),
                            2 => format!("tile_{i}_pathfindPreprocess.xml"),
                            _ => format!("tile_{i}_unknown_idx_{idx}"),
                        };
                        fs::write(output_path.join(file_name), entry.as_bytes()).unwrap();
                    }
                }

            } else {
                error!("File not found: {pepkg_path}");
            };
        },
        Some(Commands::Display { pepkg_path }) => {
            info!("Displaying {pepkg_path}");

            if std::path::Path::new(&pepkg_path).exists() {
                let pepkg = PePkg::open(&pepkg_path).expect("Failed to open pepkg file");
                display(pepkg);
            } else {
                error!("File not found: {pepkg_path}");
            };
        },
        None => {}
    }
}

#[derive(Resource)]
struct PePkgRes(PePkg);

impl Deref for PePkgRes {
    type Target = PePkg;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PePkgRes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component)]
struct FreeCamera {
    speed: f32,
    sensitivity: f32,
}

fn display(pepkg: PePkg) {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: bevy_full::render::settings::RenderCreation::Automatic(WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            WireframePlugin::default(),
        ))
        .insert_resource(PePkgRes(pepkg))
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: false,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: WHITE.into(),
        })
        .add_systems(Startup, setup_display)
        .add_systems(Update, (camera_movement, camera_look, toggle_cursor_grab))
        .run();
}

fn setup_display(
    mut pepkg: ResMut<PePkgRes>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: bevy_full::ecs::system::Commands,
    mut windows: Query<&mut Window>,
) {
    // Set initial cursor mode
    let mut window = windows.single_mut().unwrap();
    window.cursor_options.grab_mode = CursorGrabMode::None; 
    window.cursor_options.visible = true;

    info!("Setting up display");

    let federation = pepkg.read_federation_file().unwrap();
    debug!("Federation file: {federation:#?}");

    let tiles_horizontal = (federation.width as f32 / federation.tile_size as f32).ceil() as u32;
    
    // Add global directional light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            
            ..default()
        },
        Transform::from_xyz(federation.width as f32 / 2.0, federation.tile_size as f32, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Z),
    ));
    
    // Add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.3, 0.3, 0.35),
        brightness: 0.2,
        ..Default::default()
    });
    
    // Track the bounding box of all meshes
    let mut min_bounds = Vec3::splat(f32::MAX);
    let mut max_bounds = Vec3::splat(f32::MIN);

    let mut world_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
    
    let mut builder = Path::builder();

    // Spawn all tiles and calculate bounds
    for i in 0..pepkg.tile_count() {
        let (mesh, _, _) = pepkg.read_tile(i).unwrap();

        let tile_x = (mesh.tiled_cp_face_index_translator_2.federation_tile_index % tiles_horizontal) as f32 * federation.tile_size as f32;
        let tile_z = (mesh.tiled_cp_face_index_translator_2.federation_tile_index / tiles_horizontal) as f32 * federation.tile_size as f32;

        // Calculate this mesh's bounds
        for vert in &mesh.mesh_3d.verts.verts {
            let x = vert.x as f32 + tile_x;
            let y = vert.z as f32; // Height
            let z = vert.y as f32 + tile_z;
            
            min_bounds = min_bounds.min(Vec3::new(x, y, z));
            max_bounds = max_bounds.max(Vec3::new(x, y, z));
        }

        let mut tile_mesh = load_3d_mesh(&mesh);
        tile_mesh.translate_by(Vec3::new(tile_x, 0.0, tile_z));

        if world_mesh.indices().is_none() {
            world_mesh = tile_mesh;
        } else {
            world_mesh.merge(&tile_mesh).unwrap();
        }

        load_2d_mesh(&mut builder, Vec2::new(tile_x, tile_z), &mesh);

        /*let meshes_2d = load_2d_meshes(&mesh);
        for (i, mesh) in meshes_2d.into_iter().enumerate() {
            let tile_mesh = meshes.add(mesh);
            commands.spawn((
                Mesh3d(tile_mesh),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: generate_tile_color(i),
                    double_sided: true,
                    cull_mode: None,
                    perceptual_roughness: 0.8,
                    reflectance: 0.1,
                    ..Default::default()
                })),
                Transform::from_translation(Vec3::new(tile_x, 0.0, tile_z)),
            ));
        }*/
    }

    let tile_mesh = meshes.add(normalize_mesh(world_mesh));

    commands.spawn((
        Mesh3d(tile_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.8, 0.8),
            double_sided: true,
            cull_mode: None,
            perceptual_roughness: 0.8,
            reflectance: 0.1,
            ..Default::default()
        })),
    ));

    let path = builder.build();

    let mut geometry = VertexBuffers::<Vec3, u32>::new();
    let mut tesselator = FillTessellator::new();

    tesselator.tessellate_path(
        &path, 
        &FillOptions::default()
            .with_intersections(true),
        &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
            Vec3::new(vertex.position().x, 0.0, vertex.position().y)
        })
    ).unwrap();

    /*let mut tesselator = StrokeTessellator::new();
    tesselator.tessellate_path(
        &path, 
        &StrokeOptions::default().with_tolerance(0.1).with_line_width(10.0), 
        &mut BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| {
            Vec3::new(vertex.position().x, 0.0, vertex.position().y)
        })
    ).unwrap();*/

    let mut nav_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, geometry.vertices)
    .with_inserted_indices(Indices::U32(geometry.indices));

    nav_mesh.compute_normals();

    commands.spawn((
        Mesh3d(meshes.add(nav_mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.8),
            double_sided: true,
            cull_mode: None,
            perceptual_roughness: 0.8,
            reflectance: 0.1,
            ..Default::default()
        })),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));
    
    // Calculate camera position based on bounds
    let center = (min_bounds + max_bounds) * 0.5;
    let size = max_bounds - min_bounds;
    let max_dimension = size.max_element();
    
    // Position camera slightly above and to the side of the center
    let camera_position = Vec3::new(
        center.x + max_dimension * 0.3,
        center.y + max_dimension * 0.3,
        center.z + max_dimension * 0.3,
    );
    
    info!("Mesh bounds: min={min_bounds:?}, max={max_bounds:?}");
    info!("Camera position: {camera_position:?}");

    // Camera with free movement controls
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(camera_position)
            .looking_at(center, Vec3::Y),
        FreeCamera { 
            speed: 2000.0,
            sensitivity: 0.5 
        },
    ));
}

/// Generates a visually distinct color based on an index
fn generate_tile_color(index: usize) -> Color {
    // Using golden ratio for good distribution
    const GOLDEN_RATIO_CONJUGATE: f32 = 0.618_034;
    
    // Start with a seed based on index
    let mut h = (index as f32) * GOLDEN_RATIO_CONJUGATE;
    
    // Keep only fractional part
    h = h.fract();
    
    // Use HSL color model with moderate saturation and lightness
    // to avoid too bright or too dark colors
    Color::hsl(
        h * 360.0,        // Hue: 0-360 degrees around the color wheel
        0.6,              // Saturation: 60% saturation (moderate)
        0.65              // Lightness: 65% brightness (moderate)
    )
}

fn camera_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Virtual>>,
    mut query: Query<(&mut Transform, &FreeCamera)>,
) {
    if let Ok((mut transform, camera)) = query.single_mut() {
        let mut speed = camera.speed * time.delta_secs();
        
        // Calculate direction vectors once
        let forward = transform.forward();
        let right = transform.right();
        let up = Vec3::Y;

        // Speed adjustment
        if keyboard.pressed(KeyCode::KeyE) {
            speed *= 4.0; // Quadruple speed when E is pressed
        }
        
        // Forward/backward
        if keyboard.pressed(KeyCode::KeyW) {
            transform.translation += forward * speed;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            transform.translation -= forward * speed;
        }
        
        // Left/right
        if keyboard.pressed(KeyCode::KeyA) {
            transform.translation -= right * speed;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            transform.translation += right * speed;
        }
        
        // Up/down
        if keyboard.pressed(KeyCode::Space) {
            transform.translation += up * speed;
        }
        if keyboard.pressed(KeyCode::ShiftLeft) {
            transform.translation -= up * speed;
        }

    }
}

fn camera_look(
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &FreeCamera)>,
) {
    if let Ok((mut transform, camera)) = query.single_mut() {
        let mut delta = Vec2::ZERO;
        for event in mouse_motion.read() {
            delta += event.delta;
        }
        
        if delta.length_squared() > 0.0 {
            let sensitivity = camera.sensitivity;
            
            // Yaw rotation around Y axis
            transform.rotate_y(-delta.x * sensitivity * 0.01);
            
            // Pitch rotation around local X axis
            let pitch = (transform.rotation.to_euler(EulerRot::YXZ).1 - delta.y * sensitivity * 0.01)
                .clamp(-std::f32::consts::FRAC_PI_2 + 0.01, std::f32::consts::FRAC_PI_2 - 0.01);
                
            transform.rotation = Quat::from_euler(
                EulerRot::YXZ, 
                transform.rotation.to_euler(EulerRot::YXZ).0, 
                pitch,
                0.0
            );
        }
    }
}

fn toggle_cursor_grab(
    mut windows: Query<&mut Window>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let mut window = windows.single_mut().unwrap();

    // Toggle cursor grab when pressing Escape
    if keyboard.just_pressed(KeyCode::Escape) {
        match window.cursor_options.grab_mode {
            CursorGrabMode::None => {
                window.cursor_options.grab_mode = CursorGrabMode::Locked;
                window.cursor_options.visible = false;
            }
            _ => {
                window.cursor_options.grab_mode = CursorGrabMode::None;
                window.cursor_options.visible = true;
            }
        }
    }

    // Initial grab on mouse click
    if mouse.just_pressed(MouseButton::Left) && window.cursor_options.grab_mode == CursorGrabMode::None {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }
}

fn normalize_mesh(mesh: Mesh) -> Mesh {
    let mut duplicated_vecs = HashMap::new();

    let mut positions: Vec<(usize, [f32; 3])> = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap()
        .iter()
        .cloned()
        .enumerate()
        .collect();

    positions.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    debug!("Normalizing mesh with {} vertices", positions.len());

    // Find duplicated vertices
    for i in 1..positions.len() {
        if positions[i].1 == positions[i-1].1 {
            duplicated_vecs.insert(positions[i].0, positions[i-1].0);
        }
    }

    let mut index_map = HashMap::new();

    let mut new_index = 0;
    let positions = positions
        .into_iter()
        .filter_map(|(original_index, vert)| {
            if let Some(&replaced_index) = duplicated_vecs.get(&original_index) {
                let replaced_index = index_map.get(&replaced_index)
                    .cloned()
                    .unwrap();

                index_map.insert(original_index, replaced_index);
                None
            } else {
                index_map.insert(original_index, new_index);
                new_index += 1;
                Some(vert)
            }
        })
        .collect::<Vec<_>>();

    debug!("Found {} duplicated vertices", duplicated_vecs.len());
    debug!("New vertex count: {}", positions.len());

    let mut indices = mesh.indices().unwrap()
        .iter()
        .map(|index| {
            if let Some(&new_index) = index_map.get(&index) {
                new_index as u32
            } else {
                index as u32
            }
        })
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            [chunk[0], chunk[1], chunk[2]]
        })
        .collect::<Vec<_>>();

    indices.dedup();

    let indices = indices
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    debug!("Normalized mesh");

    let mut mesh = Mesh::new(mesh.primitive_topology(), mesh.asset_usage)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_indices(Indices::U32(indices));
    
    mesh.compute_normals();

    mesh
}

fn load_3d_mesh(mesh: &pepkg::Mesh) -> Mesh {
    // Convert vertices first to have them available for normal calculation
    let mut positions: Vec<[f32; 3]> = mesh.mesh_3d.verts.verts
        .iter()
        .map(|v| [v.x as f32, v.z as f32, v.y as f32])
        .collect();
    
    // Convert indices
    let indices: Vec<u32> = mesh.mesh_3d.tris.tris
        .iter()
        .flat_map(|t| {
            let vert0 = if let Some(start_z) = t.edge0_start_z {
                let pos = positions.get(t.edge0_start_vert as usize).unwrap();

                positions.push([pos[0], start_z as f32, pos[2]]);
                positions.len() as u32 - 1
            } else {
                t.edge0_start_vert
            };

            let vert1 = if let Some(start_z) = t.edge1_start_z {
                let pos = positions.get(t.edge1_start_vert as usize).unwrap();

                positions.push([pos[0], start_z as f32, pos[2]]);
                positions.len() as u32 - 1
            } else {
                t.edge1_start_vert
            };

            let vert2 = if let Some(start_z) = t.edge2_start_z {
                let pos = positions.get(t.edge2_start_vert as usize).unwrap();

                positions.push([pos[0], start_z as f32, pos[2]]);
                positions.len() as u32 - 1
            } else {
                t.edge2_start_vert
            };
            
            [vert0, vert1, vert2]
        })
        .collect();
    
    // Create the mesh with positions, normals, and indices
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_indices(Indices::U32(indices));

    mesh.compute_normals();
    
    mesh
}

fn load_2d_mesh(builder: &mut NoAttributes<BuilderImpl>, pos: Vec2, mesh: &pepkg::Mesh) {
    let verts = mesh.mesh_3d.verts.verts.as_slice();
    let poligons = mesh.mapping_to_2d.polys.as_slice();

    for polygon in poligons {
        if 
            let Some(edge) = polygon.edges.first() &&
            let Some(vert) = verts.get(edge.start_vert as usize)
        {
            builder.begin(point(vert.x as f32 + pos.x, vert.y as f32 + pos.y));
        } else {
            break;
        }

        for edge in polygon.edges.iter().skip(1) {
            if let Some(vert) = verts.get(edge.start_vert as usize) {
                builder.line_to(point(vert.x as f32 + pos.x, vert.y as f32 + pos.y));
            }
        }

        builder.close();
    }
}

fn load_2d_meshes(mesh: &pepkg::Mesh) -> Vec<Mesh> {
    let mut meshes = Vec::new();

    let verts = mesh.mesh_3d.verts.verts.as_slice();
    let poligons = mesh.mapping_to_2d.polys.as_slice();


    for polygon in poligons {
        let mut builder = Path::builder();

        if 
            let Some(edge) = polygon.edges.first() &&
            let Some(vert) = verts.get(edge.start_vert as usize)
        {
            builder.begin(point(vert.x as f32, vert.y as f32));
        } else {
            break;
        }

        for edge in polygon.edges.iter().skip(1) {
            if let Some(vert) = verts.get(edge.start_vert as usize) {
                builder.line_to(point(vert.x as f32, vert.y as f32));
            }
        }

        builder.end(true);

        let mut geometry = VertexBuffers::<Vec3, u32>::new();
    
        let mut tesselator = StrokeTessellator::new();
        tesselator.tessellate_path(
            &builder.build(), 
            &StrokeOptions::default().with_tolerance(0.1).with_line_width(10.0), 
            &mut BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| {
                Vec3::new(vertex.position().x, 0.0, vertex.position().y)
            })
        ).unwrap();

        let mut result_mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, geometry.vertices)
        .with_inserted_indices(Indices::U32(geometry.indices));

        result_mesh.compute_normals();

        meshes.push(result_mesh);
    }

    meshes
}
