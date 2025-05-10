use std::{fs, ops::{Deref, DerefMut}, path::Path};

use bevy_full::{asset::RenderAssetUsages, color::palettes::css::WHITE, input::mouse::MouseMotion, pbr::wireframe::{WireframeConfig, WireframePlugin}, prelude::*, render::{mesh::Indices, settings::{WgpuFeatures, WgpuSettings}, RenderPlugin}, window::CursorGrabMode};
use clap::{Parser, Subcommand};
use log::{error, info};
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

            if Path::new(&pepkg_path).exists() {
                let mut pepkg = PePkg::open(&pepkg_path).expect("Failed to open pepkg file");
                let output_path = Path::new(&output_path);
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

            if Path::new(&pepkg_path).exists() {
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

        let tile_mesh = meshes.add(load_pe_mesh(&mesh));

        // Generate a unique color based on tile index
        let tile_color = generate_tile_color(i);
        
        commands.spawn((
            Mesh3d(tile_mesh),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: tile_color,
                double_sided: true,
                cull_mode: None,
                perceptual_roughness: 0.8,
                reflectance: 0.1,
                


                ..Default::default()
            })),
            Transform::from_xyz(tile_x, 0.0, tile_z),
        ));
    }
    
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

fn load_pe_mesh(mesh: &pepkg::Mesh) -> Mesh {
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
    
    // Calculate normals
    let mut normals = vec![[0.0, 0.0, 0.0]; positions.len()];
    let mut counts = vec![0; positions.len()];
    
    // Process each triangle
    for chunk in indices.chunks(3) {
        if chunk.len() == 3 {
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;
            
            // Get vertices of this triangle
            if i0 >= positions.len() || i1 >= positions.len() || i2 >= positions.len() {
                continue; // Skip invalid indices
            }
            
            let v0 = Vec3::from(positions[i0]);
            let v1 = Vec3::from(positions[i1]);
            let v2 = Vec3::from(positions[i2]);
            
            // Calculate triangle edges
            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            
            // Calculate face normal using cross product
            let normal = edge1.cross(edge2).normalize();
            
            // Add this normal to all vertices of this face
            normals[i0][0] += normal.x;
            normals[i0][1] += normal.y;
            normals[i0][2] += normal.z;
            counts[i0] += 1;
            
            normals[i1][0] += normal.x;
            normals[i1][1] += normal.y;
            normals[i1][2] += normal.z;
            counts[i1] += 1;
            
            normals[i2][0] += normal.x;
            normals[i2][1] += normal.y;
            normals[i2][2] += normal.z;
            counts[i2] += 1;
        }
    }
    
    // Average and normalize all vertex normals
    for i in 0..normals.len() {
        if counts[i] > 0 {
            normals[i][0] /= counts[i] as f32;
            normals[i][1] /= counts[i] as f32;
            normals[i][2] /= counts[i] as f32;
            
            // Normalize
            let length = (normals[i][0] * normals[i][0] + 
                          normals[i][1] * normals[i][1] + 
                          normals[i][2] * normals[i][2]).sqrt();
            if length > 0.0 {
                normals[i][0] /= length;
                normals[i][1] /= length;
                normals[i][2] /= length;
            } else {
                // Default normal if calculation failed
                normals[i] = [0.0, 1.0, 0.0];
            }
        } else {
            // Default normal for vertices not used in any triangle
            normals[i] = [0.0, 1.0, 0.0];
        }
    }
    
    // Create the mesh with positions, normals, and indices
    Mesh::new(
        bevy_full::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_indices(Indices::U32(indices))
}

