use bevy::prelude::*;

use noise::NoiseFn;
use noise::OpenSimplex;

use rand::Rng;
use rand::thread_rng;


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
        .add_startup_system(spawn_chunk.system())

        .run();
}

/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
) {
    commands
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),
            ..Default::default()
        })
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-31.0, 60.0, -5.0),
            )),
            ..Default::default()
        });
}

fn spawn_chunk(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    seed: Res<Seed>,
) {
    let noise = OpenSimplex::new();
    let chunk_size = 8;

    for chunk_x in -2 .. 2 { 
    for chunk_z in -2 .. 2 {

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane{ size: 1.0 })),
                material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
                transform: Transform::from_translation(Vec3::new((chunk_x * chunk_size) as f32, 0.0, (chunk_z * chunk_size) as f32)),
                ..Default::default()
            })
            .with(Chunk { x: chunk_x, z: chunk_z })
            .with_children(|parent| {

            let texture_handle = asset_server.load("textures/dirt.png");
    
            for x in -chunk_size .. chunk_size {
            for z in -chunk_size .. chunk_size {
                let y = (noise.get([
                    ( (x + chunk_x * chunk_size) as f32 / 20. ) as f64, 
                    ( (z + chunk_z * chunk_size) as f32 / 20. ) as f64,
                    seed.value,
                ]) * 15. + 16.0) as u32;


                parent
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube{ size: 1.0 })),
                        material: materials.add(StandardMaterial { albedo: Color::rgba(1.0, 1.0, 1.0, 1.0), albedo_texture: Some(texture_handle.clone()), ..Default::default() }),
                        transform: Transform::from_translation(Vec3::new(x as f32, y as f32, z as f32)),
                        ..Default::default()
                    })
                .with(Cube);
            }
            }
        });
    }}
}