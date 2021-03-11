use bevy::prelude::*;

struct Cube;

struct Rotation {
    angle: f32,
}

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 16 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Voxel!".to_string(),
            width: 1600.,
            height: 1600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())

        .add_system(rotate_cube.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        // Plane
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 3.0 })),
            material: materials.add(Color::rgb(1., 0.5, 0.5).into()),
            transform: Transform::from_translation(Vec3::new(5., 0., 5.)),
            ..Default::default()
        })
        .with(Cube)
        .with(Rotation { angle: 0.0 })

        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            )),
            ..Default::default()
        })

        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}

fn rotate_cube(
    mut cube: Query<(&mut Transform, &mut Rotation), With<Cube>>,
    time: Res<Time>,
) {
    for (mut cube, mut rotation) in cube.iter_mut() {
        rotation.angle += 1.0 * time.delta_seconds();
        cube.rotation = Quat::from_rotation_y(rotation.angle);
    }
}