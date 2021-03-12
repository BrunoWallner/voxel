use bevy::prelude::*;

use bevy_flycam::PlayerPlugin;
use bevy_flycam::MovementSettings;

use noise::NoiseFn;
use noise::OpenSimplex;

use rand::Rng;
use rand::thread_rng;


struct Cube;
struct Chunk;

struct Seed {value: f64}

fn main() {
    App::build()
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {title: "Voxel!".to_string(), width: 1200.0, height: 800.0, ..Default::default()})
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_resource(MovementSettings {
            sensitivity: 0.00005, // default: 0.00012
            speed: 15.0, // default: 12.0
        })

        .add_resource(Seed { value: thread_rng().gen() })
        .add_startup_system(setup.system())
        .add_startup_system(spawn_chunk.system())

        .add_system(rotate_chunk.system())

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
        });
}

fn spawn_chunk(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    seed: Res<Seed>,
) {
    let noise = OpenSimplex::new();

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane{ size: 1.0 })),
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(Chunk)
        .with_children(|parent| {
    
        for x in -32 .. 32 {
            for z in -32 .. 32 {
                let y = (noise.get([
                    ( x as f32 / 20. ) as f64, 
                    ( z as f32 / 20. ) as f64,
                    seed.value,
                ]) * 15. + 16.0) as u32;


                parent
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube{ size: 1.0 })),
                        material: materials.add(Color::rgb(1., 0.5, 0.5).into()),
                        transform: Transform::from_translation(Vec3::new(x as f32, y as f32, z as f32)),
                        ..Default::default()
                    })
                .with(Cube);
            }
        }
    });
}

fn rotate_chunk(
    mut chunks: Query<&mut Transform, With<Chunk>>,
    time: Res<Time>,
) {
    for mut chunk in chunks.iter_mut() {
        chunk.rotation *= Quat::from_rotation_y(0.25 * time.delta_seconds());
    }

}