use bevy::prelude::*;
use bevy::render::pipeline::PrimitiveTopology;

use noise::NoiseFn;
use noise::OpenSimplex;
use noise::Seedable;

use rand::Rng;
use rand::thread_rng;

struct Materials {
    blocks: Vec<Handle<StandardMaterial>>,
}

struct Camera;

struct Chunk {
    pub x: i32,
    pub z: i32,
    pub index: [[[u8; 16]; 256]; 16]
}
impl Chunk {
    //creates a new empty chunk filled with air
    pub fn new(x: i32, z: i32) -> Self {
        Chunk {x: x, z: z, index: [[[0u8; 16]; 256]; 16]}
    }
}

struct Seed {value: u32}

#[bevy_main]
fn main() {
    App::build()
        .add_resource(WindowDescriptor {title: "Voxel!".to_string(), width: 1200.0, height: 800.0, ..Default::default()})
        .add_plugins(DefaultPlugins)

        .add_resource(Seed { value: thread_rng().gen_range(0 .. 4294967295) })

        .add_startup_system(setup.system())

        .add_startup_stage("ChunkCreation", SystemStage::serial())
        .add_startup_system_to_stage("ChunkCreation",create_chunk.system())

        .add_startup_stage_after("ChunkCreation", "ChunkGeneration", SystemStage::serial())
        .add_startup_system_to_stage("ChunkGeneration",generate_chunk.system())

        .add_startup_stage_after("ChunkGeneration", "ChunkSpawning", SystemStage::serial())
        .add_startup_system_to_stage("ChunkSpawning",spawn_chunk.system())

        .add_system(camera_controll.system())

        .run();
}

/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(100.0, 10000000.0, -120.0)),
            ..Default::default()
        })
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.1, -0.5, -0.1, 0.5).normalize(),
                Vec3::new(-10.0, 35.0, 0.0),
            )),
            ..Default::default()
        })
        .with(Camera);

        let grass_material_handle = asset_server.load("textures/grass.png");
        let stone_material_handle = asset_server.load("textures/stone.png");

        let air = materials.add(StandardMaterial { albedo: Color::rgba(1.0, 1.0, 1.0, 1.0), ..Default::default() }); 
        let grass = materials.add(StandardMaterial { albedo: Color::rgba(1.0, 1.0, 1.0, 1.0), albedo_texture: Some(grass_material_handle.clone()), ..Default::default() });
        let stone = materials.add(StandardMaterial { albedo: Color::rgba(1.0, 1.0, 1.0, 1.0), albedo_texture: Some(stone_material_handle.clone()), ..Default::default() }); 

        let mut blocks: Vec<Handle<StandardMaterial>> = vec![];
        blocks.push(air);
        blocks.push(grass);
        blocks.push(stone);

        commands.insert_resource(Materials {
            blocks: blocks,
        });
}

fn camera_controll(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let speed = 10.0;
    for mut camera in camera.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            camera.translation.x += speed * time.delta_seconds()
        }
        if keyboard_input.pressed(KeyCode::A) {
            camera.translation.z -= speed * time.delta_seconds()
        }
        if keyboard_input.pressed(KeyCode::S) {
            camera.translation.x -= speed * time.delta_seconds()
        }
        if keyboard_input.pressed(KeyCode::D) {
            camera.translation.z += speed * time.delta_seconds()
        }
        if keyboard_input.pressed(KeyCode::Space) {
            camera.translation.y += speed * time.delta_seconds()
        }
        if keyboard_input.pressed(KeyCode::LShift) {
            camera.translation.y -= speed * time.delta_seconds()
        }
    }
}

fn create_chunk(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Materials>,
) {
    let chunk_size = 16;

    for chunk_x in -1 .. 1 { 
        for chunk_z in -1 .. 1 {

            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane{ size: 1.0 })),
                    material: materials.blocks[1].clone(),
                    transform: Transform::from_translation(Vec3::new((chunk_x * chunk_size) as f32, 0.0, (chunk_z * chunk_size) as f32)),
                    ..Default::default()
                })
                .with(Chunk::new(chunk_x, chunk_z));
            }
        }
}

fn generate_chunk(
    mut chunks: Query<&mut Chunk, With<Chunk>>,
    seed: Res<Seed>,
) {
    let chunk_size: usize = 16;
    let noise = OpenSimplex::new();
    noise.set_seed(seed.value);

    for mut chunk in chunks.iter_mut() {
        for x in 0 .. chunk_size {
            for z in 0 .. chunk_size {
                
                let rng = thread_rng().gen_range(1 .. 10000);

                let y = (noise.get([
                    ( (x as i32 + chunk.x * chunk_size as i32) as f32 / 20. ) as f64, 
                    ( (z as i32 + chunk.z * chunk_size as i32) as f32 / 20. ) as f64,
                ]) * 20. + 16.0) as usize;

                /*
                for i in 0 .. y - 4 {
                    chunk.index[x][i][z] = 2;
                }
                for i in y - 4 .. y  {
                    chunk.index[x][i][z] = 1;
                }
                */
                chunk.index[x][y][z] = 1;

                //generates random stones on surface
                if rng & 5000 == 0 {
                    chunk.index[x][y][z] = 2;
                }
            }
        }
    }
}

use bevy::render::mesh::Indices;

fn spawn_chunk(
    commands: &mut Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    chunks: Query<&Chunk, With<Chunk>>,
    time: Res<Time>,
) {
    let start_time = time.time_since_startup();
    let mut chunk_counter: u32 = 0;
    for chunk in chunks.iter() {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();

        let mut indices: Vec<u32> = Vec::new();

        //creates all the needed positions vor vertices
        for x in 0..17 {
            for y in 0..257 {
                for z in 0..17 {
                    positions.push([x as f32, y as f32, z as f32]);
                    normals.push([0., 0., 0.]);
                    uvs.push([0., 0.]);
                }
            }
        }
        
        for x in 0..16 {
            for mut y in 0..256 {
                for z in 0..16 {
                    if chunk.index[x][y][z] != 0 {
                        indices.push(get_indice_position([x+1, y, z  ]));
                        indices.push(get_indice_position([x  , y, z  ]));
                        indices.push(get_indice_position([x+1, y, z+1]));
                        
                    }
                }
            }
        }


        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        mesh.set_indices(Some(Indices::U32(indices)));


        commands
            .spawn(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
                transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                    Vec3::splat(1.0),
                    Quat::from_rotation_x(0.0),
                    Vec3::new((chunk.x * 16) as f32, 0.0, (chunk.z * 16) as f32),
                )),
                ..Default::default()
            });
        chunk_counter+=1;
    }
    let end_time = time.time_since_startup() - start_time;
    
    println!("Generated {} chunks in {:?} seconds", chunk_counter, end_time);
}

fn get_indice_position(
    position: [usize; 3],
) -> u32 {
    let x = 17; //chunk sizes
    let y = 257;

    (position[0]*x*y + position[1]*x + position[2]) as u32
}