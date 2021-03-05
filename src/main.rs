use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, Input};

struct Textures {
    ship_texture: Handle<TextureAtlas>
}

struct Rotation {
    speed: f32,
    angle: f32,
}

struct Ship;


pub fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(WindowDescriptor { title: "Space Shooter".to_string(), width:1200.0, height: 800.0, ..Default::default() })
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_ship.system()))
        .add_system(ship_movement.system())
        .add_plugins(DefaultPlugins)
    
    .run();
}

fn setup(
    commands: &mut Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("textures/ship.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 1, 1);

    commands.insert_resource(Textures {
        ship_texture: texture_atlases.add(texture_atlas),
    });
}

fn spawn_ship(commands: &mut Commands, texture: Res<Textures>) {
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.ship_texture.clone(),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .with(Ship)
        .with(Rotation { angle: 0.0, speed: 0.1 });
}

fn ship_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut ship_position: Query<(&mut Transform, &mut Rotation), With<Ship>>,
) {
    for (mut transform, mut rotation) in ship_position.iter_mut() {
        let move_dir = transform.rotation * Vec3::unit_y();
        let move_speed = 15.0;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            rotation.angle += rotation.speed;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            rotation.angle -= rotation.speed;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            transform.translation -= move_dir * move_speed;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            transform.translation += move_dir * move_speed;
        }

        //applies rotation
        transform.rotation = Quat::from_rotation_z(rotation.angle);
    }
}


