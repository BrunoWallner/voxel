use bevy::prelude::*;

pub struct Materials {
    pub blocks: Handle<StandardMaterial>,
}


mod chunk;
mod input;

pub struct Camera;
pub struct Light;

#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)

        .add_startup_system(setup.system())
        .add_startup_system(chunk::spawn_world.system())

        .add_startup_stage("spawn", SystemStage::single(chunk::generate_spawn.system()))

        .add_startup_stage("render", SystemStage::single(chunk::render_chunk.system()))

        .add_system(input::build.system())
        .add_system(input::movement.system())
        .add_system(input::builder_movement.system())

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
        .insert_resource(WindowDescriptor {title: "Voxel!".to_string(), width: 1200.0, height: 800.0, ..Default::default()})
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa { samples: 1 })
        
        // Light
        .spawn(LightBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(0.0, 0.0, 0.0, 0.0).normalize(),
                Vec3::new(0.0, 1000000000.0, 0.0),
            )),
            ..Default::default()
        })
        .with(Light);


        /* Camera
        let mut camera = OrthographicCameraBundle::new_3d();
        camera.orthographic_projection.scale = 50.0;
        camera.transform = Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::unit_y());
        camera.transform.translation = Vec3::new(100.0, 100.0, 100.0);
        
        commands.spawn(camera).with(Camera);
        */

        commands
            .spawn(PerspectiveCameraBundle {
                transform: Transform::from_xyz(-75.0, 100.0, -75.0)
                    .looking_at(Vec3::zero(), Vec3::unit_y()),
                ..Default::default()
            })
            .with(Camera);

        let block_texture_handle = asset_server.load("textures/blocks.png");

        let blocks = materials.add(StandardMaterial { albedo: Color::rgba(1.0, 1.0, 1.0, 1.0), albedo_texture: Some(block_texture_handle.clone()), ..Default::default() });

        commands.insert_resource(Materials {
            blocks: blocks,
        });
}
