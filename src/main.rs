use bevy::prelude::*;

pub struct Materials {
    pub blocks: Handle<StandardMaterial>,
}

mod chunk;
mod controll;
mod player_input;

use player_input::*;

pub struct Camera;
pub struct Light;

#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)

        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.025, // default: 0.00012
            speed: 25.0, // default: 12.0
        })

        .add_startup_system(setup.system())
        .add_startup_system(chunk::spawn_world.system())

        .add_startup_stage("spawn", SystemStage::single(chunk::generate_spawn.system()))

        .add_startup_stage("render", SystemStage::single(chunk::render_chunk.system()))

        .add_system(controll::build.system())
        .add_system(controll::movement.system()) // syncs light position to builder
        .add_system(controll::builder_movement.system())

        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Window settings
    commands.insert_resource(WindowDescriptor {title: "Voxel!".to_string(), width: 1200.0, height: 800.0, vsync: false, ..Default::default()});
        
    // Clear Color    
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
        
    // Light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 65.0, 0.0)),
        light: bevy::pbr::Light {
            color: Color::rgb(1.0, 1.0, 1.0),
            fov: 360.0,
            range: 10000.0,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Light);


    // spawns player
    commands.spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 65.0, 0.0),
            ..Default::default()
        })
        .insert(Camera)
        .insert(FlyCam);

    let block_texture_handle = asset_server.load("textures/blocks.png");

    let blocks = materials.add(StandardMaterial { 
        base_color: Color::rgba(1.0, 1.0, 1.0, 1.0), 
        base_color_texture: Some(block_texture_handle.clone()), 
        roughness: 2.5,
        metallic: 0.1,
        ..Default::default() });

    commands.insert_resource(Materials {
        blocks: blocks,
    });

    // spawn builderindicator
    let builder_texture_handle = asset_server.load("textures/builder.png");

    let builder_texture = materials.add(StandardMaterial { 
        base_color: Color::rgba(1.0, 1.0, 1.0, 1.0), 
        base_color_texture: Some(builder_texture_handle.clone()),
        roughness: 0.5,
        metallic: 5.0,
        ..Default::default() });

    commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: builder_texture.clone(),
            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::splat(0.75),
                Quat::from_rotation_x(0.0),
                Vec3::new(0.0, 0.0, 0.0),
            )),
                ..Default::default()
        })
        .insert(controll::BuilderIndicator);
}
