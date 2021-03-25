use bevy::prelude::*;


use bevy_flycam::NoCameraPlayerPlugin;
use bevy_flycam::FlyCam;
use bevy_flycam::MovementSettings;

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

        .add_plugin(NoCameraPlayerPlugin)
        .add_resource(MovementSettings {
            sensitivity: 0.000020, // default: 0.00012
            speed: 15.0, // default: 12.0
        })

        .add_startup_system(setup.system())

        //.add_system(camera_controll.system())
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
            transform: Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),
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
        .with(Camera)
        .with(FlyCam);
        

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
