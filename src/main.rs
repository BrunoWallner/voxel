use bevy::prelude::*;

use noise::NoiseFn;
use noise::OpenSimplex;

use rand::Rng;
use rand::thread_rng;

struct Materials {
    grass: Handle<StandardMaterial>,
    redstone: Handle<StandardMaterial>,
}

struct Cube;
struct Chunk {
    pub x: i32,
    pub z: i32,
    pub index: [[[u8; 16]; 256]; 16]
}
impl Chunk {
    //creates a new empty chunk filld with air
    pub fn new(x: i32, z: i32) -> Self {
        Chunk {x: x, z: z, index: [[[0u8; 16]; 256]; 16]}
    }
}

struct Seed {value: f64}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {title: "Voxel!".to_string(), width: 1200.0, height: 800.0, ..Default::default()})
        .add_plugins(DefaultPlugins)

        .add_resource(Seed { value: thread_rng().gen() })

        .add_startup_system(setup.system())

        .add_startup_stage("ChunkCreation", SystemStage::serial())
        .add_startup_system_to_stage("ChunkCreation",create_chunk.system())

        .add_startup_stage_after("ChunkCreation", "ChunkGeneration", SystemStage::serial())
        .add_startup_system_to_stage("ChunkGeneration",generate_chunk.system())

        .add_startup_stage_after("ChunkGeneration", "ChunkSpawning", SystemStage::serial())
        .add_startup_system_to_stage("ChunkSpawning",spawn_chunk.system())


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
            transform: Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),
            ..Default::default()
        })
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.15, -0.5, -0.15, 0.5).normalize(),
                Vec3::new(-125.0, 75.0, -8.0),
            )),
            ..Default::default()
        });

        let grass_material_handle = asset_server.load("textures/grass.png");
        let redstone_material_handle = asset_server.load("textures/redstone.png");

        commands.insert_resource(Materials {
            grass: materials.add(StandardMaterial { albedo: Color::rgba(1.0, 1.0, 1.0, 1.0), albedo_texture: Some(grass_material_handle.clone()), ..Default::default() }), 
            redstone: materials.add(StandardMaterial { albedo: Color::rgba(1.0, 1.0, 1.0, 1.0), albedo_texture: Some(redstone_material_handle.clone()), ..Default::default() }), 
        });

}

fn create_chunk(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Materials>,
) {
    let chunk_size = 16;

    for chunk_x in -4 .. 4 { 
        for chunk_z in -4 .. 4 {

            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane{ size: 1.0 })),
                    material: materials.grass.clone(),
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

    for mut chunk in chunks.iter_mut() {
        for x in 0 .. chunk_size {
            for z in 0 .. chunk_size {
                
                let rng = thread_rng().gen_range(1 .. 10000);

                let y = (noise.get([
                    ( (x as i32 + chunk.x * chunk_size as i32) as f32 / 20. ) as f64, 
                    ( (z as i32 + chunk.z * chunk_size as i32) as f32 / 20. ) as f64,
                    seed.value * 10000.0,
                ]) * 20. + 5.0) as usize;

                chunk.index[x][y][z] = 1;

            }
        }
        
    }
}

fn spawn_chunk(
    commands: &mut Commands,
    chunks: Query<&Chunk, With<Chunk>>,
    materials: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for chunk in chunks.iter() {

        for x in 0 .. 16 {
            for y in 0 .. 255 {
                for z in 0 .. 16 {

                    if chunk.index[x][y][z] == 1 {
                        commands
                            .spawn(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Cube{ size: 1.0 })),
                                material: materials.grass.clone(),
                                transform: Transform::from_translation(Vec3::new((x as i32 + chunk.x * 16) as f32, y as f32, (z as i32 + chunk.z * 16) as f32)),
                                ..Default::default()
                            })
                            .with(Cube);
                    }
                }    
            }
        }
    }  
}
