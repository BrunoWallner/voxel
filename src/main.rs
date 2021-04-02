use bevy::prelude::*;


use bevy_flycam::NoCameraPlayerPlugin;
use bevy_flycam::FlyCam;
use bevy_flycam::MovementSettings;

pub struct Materials {
    pub blocks: Handle<StandardMaterial>,
}


mod chunk;

pub struct Camera;
pub struct Light;

#[bevy_main]
fn main() {
    App::build()
        .add_resource(WindowDescriptor {title: "Voxel!".to_string(), width: 1200.0, height: 800.0, ..Default::default()})

        .add_plugins(DefaultPlugins)

        .add_plugin(NoCameraPlayerPlugin)
        .add_resource(MovementSettings {
            sensitivity: 0.000020, // default: 0.00012
            speed: 25.0, // default: 12.0
        })

        .add_startup_system(setup.system())
        .add_startup_system(chunk::spawn_world.system())

        .add_startup_stage("spawn", SystemStage::serial())
        .add_startup_system_to_stage("spawn", chunk::generate_spawn.system())

        .add_startup_stage("render", SystemStage::serial())
        .add_startup_system_to_stage("render", chunk::render_chunk.system())

        .run();
}

/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        // Clear Color
        .insert_resource(ClearColor(Color::rgb(0.0, 0.2, 0.1)))
        
        // Light
        .spawn(LightBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(0.0, 0.0, 0.0, 0.0).normalize(),
                Vec3::new(0.0, 1000000000.0, 0.0),
            )),
            ..Default::default()
        })
        .with(Light)

        .spawn(LightBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(0.0, 0.0, 0.0, 0.0).normalize(),
                Vec3::new(0.0, -1000000000.0, 0.0),
            )),
            ..Default::default()
        })
        .with(Light)

        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(0.0, 0.0, 0.0, 0.5).normalize(),
                Vec3::new(-200.0, 25.0, 0.0),
            )),
            ..Default::default()
        })
        .with(Camera)
        .with(FlyCam);
        

        let block_texture_handle = asset_server.load("textures/blocks.png");

        let blocks = materials.add(StandardMaterial { albedo: Color::rgba(1.0, 1.0, 1.0, 1.0), albedo_texture: Some(block_texture_handle.clone()), ..Default::default() });

        commands.insert_resource(Materials {
            blocks: blocks,
        });
}
