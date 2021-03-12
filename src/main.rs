use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, Input};

use rand::Rng;

struct Cube;
struct Camera;

struct Rotation { 
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    App::build()
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {title: "Voxel!".to_string(), width: 1200.0, height: 800.0, ..Default::default()})
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(spawn_cubes.system())

        .add_system(rotate_cube.system())
        .add_system(camera_controll.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
) {
    commands
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(4., 0., 4.)),
            ..Default::default()
        })
        .with(Camera)
        .with(Rotation { x: 0.0, y: 20.0, z: 0.0})

        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}

fn spawn_cubes(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for _ in 0 .. 1000 {
        let x = rand::thread_rng().gen_range(-100. .. 100.);
        let y = rand::thread_rng().gen_range(-100. .. 100.);
        let z = rand::thread_rng().gen_range(-100. .. 100.);

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 3.0 })),
                material: materials.add(Color::rgb(1., 0.5, 0.5).into()),
                transform: Transform::from_translation(Vec3::new(x, y, z)),
                ..Default::default()
            })
            .with(Cube)
            .with(Rotation { x: 0.0, y: 0.0, z: 0.0 });
    }
}

fn rotate_cube(
    mut cube: Query<(&mut Transform, &mut Rotation), With<Cube>>,
    time: Res<Time>,
) {
    for (mut cube, mut rotation) in cube.iter_mut() {
        rotation.y += 1.0 * time.delta_seconds();
        cube.rotation = Quat::from_rotation_y(rotation.y);
    }
}

fn camera_controll(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera: Query<(&mut Transform, &mut Rotation), With<Camera>>,
    time: Res<Time>,
) {
    for (mut camera, mut rotation) in camera.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            rotation.y += 1.0 * time.delta_seconds();
            camera.rotation = Quat::from_rotation_y(rotation.y);
        }
        if keyboard_input.pressed(KeyCode::D) {
            rotation.y -= 1.0 * time.delta_seconds();
            camera.rotation = Quat::from_rotation_y(rotation.y);
        }
    }
}