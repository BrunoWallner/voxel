use bevy::prelude::*;

pub struct Materials {
    pub blocks: Handle<StandardMaterial>,
}


mod chunk;
mod controll;
mod player_input;

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

        .add_system(controll::build.system())
        .add_system(controll::movement.system())
        .add_system(controll::builder_movement.system())

        .run();
}

/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
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

        // spawns player
        commands
            .spawn(PerspectiveCameraBundle {
                transform: Transform::from_xyz(-75.0, 75.0, -75.0)
                    .looking_at(Vec3::zero(), Vec3::unit_y()),
                ..Default::default()
            })
            .with(Camera);

        let block_texture_handle = asset_server.load("textures/blocks.png");

        let blocks = materials.add(StandardMaterial { albedo: Color::rgba(1.0, 1.0, 1.0, 1.0), albedo_texture: Some(block_texture_handle.clone()), ..Default::default() });

        commands.insert_resource(Materials {
            blocks: blocks,
        });

        // spawn builderindicator
        let builder_texture_handle = asset_server.load("textures/builder.png");
        let builder_texture = materials.add(StandardMaterial { albedo: Color::rgba(1.0, 1.0, 1.0, 1.0), albedo_texture: Some(builder_texture_handle.clone()), ..Default::default() });

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: builder_texture.clone(),
                transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                    Vec3::splat(0.75),
                    Quat::from_rotation_x(0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                )),
                    ..Default::default()
            })
            .with(controll::BuilderIndicator);
}
