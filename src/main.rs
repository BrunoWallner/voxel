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
    x: i32,
    z: i32,
}

struct Seed {value: f64}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {title: "Voxel!".to_string(), width: 1200.0, height: 800.0, ..Default::default()})
        .add_plugins(DefaultPlugins)

        .add_resource(Seed { value: thread_rng().gen() })

        .add_startup_system(setup.system())

        .add_startup_stage("ChunkSpawn", SystemStage::serial())
        .add_startup_system_to_stage("ChunkSpawn",spawn_chunk.system())

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
                Vec3::new(-125.0, 75.0, -10.0),
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

fn spawn_chunk(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    seed: Res<Seed>,
    materials: Res<Materials>,
) {
    let noise = OpenSimplex::new();
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
            .with(Chunk { x: chunk_x, z: chunk_z })
            .with_children(|parent| {
    
            for x in -chunk_size .. chunk_size {
            for z in -chunk_size .. chunk_size {

                let rng = thread_rng().gen_range(1 .. 10000);

                let y = (noise.get([
                    ( (x + chunk_x * chunk_size) as f32 / 20. ) as f64, 
                    ( (z + chunk_z * chunk_size) as f32 / 20. ) as f64,
                    seed.value * 10000.0,
                ]) * 15. + 16.0) as u32;


                parent
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube{ size: 1.0 })),
                        material: materials.grass.clone(),
                        transform: Transform::from_translation(Vec3::new(x as f32, y as f32, z as f32)),
                        ..Default::default()
                    })
                .with(Cube);

                if rng % 2000 == 0 {
                    parent
                        .spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube{ size: 1.0 })),
                            material: materials.redstone.clone(),
                            transform: Transform::from_translation(Vec3::new(x as f32, y as f32 + 1.0, z as f32)),
                            ..Default::default()
                        })
                    .with(Cube);
                }
            }
            }
        });
    }}
}