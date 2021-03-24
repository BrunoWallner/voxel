use bevy::prelude::*;

pub struct Materials {
    pub blocks: Vec<Handle<StandardMaterial>>,
}

mod chunk;

pub struct Camera;

#[bevy_main]
fn main() {
    App::build()
        .add_resource(WindowDescriptor {title: "Voxel!".to_string(), width: 1200.0, height: 800.0, ..Default::default()})
        .add_plugins(DefaultPlugins)

        .add_startup_system(setup.system())

        .add_startup_stage("ChunkCreation", SystemStage::serial())
        .add_startup_system_to_stage("ChunkCreation",chunk::create_chunk.system())

        .add_system(camera_controll.system())
        .add_system(chunk::chunk_loader.system())
        .add_system(chunk::spawn_chunk.system())

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
                Quat::from_xyzw(-0.5, -0.5, -0.5, 0.5).normalize(),
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
    let speed = 25.0;
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