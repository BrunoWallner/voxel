use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, Input};

use rand::Rng;

struct Textures {
    ship_texture: Handle<TextureAtlas>,
    background_layer1_texture: Handle<TextureAtlas>,
    background_layer2_texture: Handle<TextureAtlas>,
    background_layer3_texture: Handle<TextureAtlas>,
    comet_texture: Handle<TextureAtlas>,
}

struct Rotation {
    angle: f32,
}

struct Layer {
    value: u32,
}

struct ShipStatus {
    pub dead: bool,
}

struct Ship;
struct Background;
struct Camera;
struct Comet;

pub fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(WindowDescriptor { title: "Space Shooter".to_string(), width:1200.0, height: 800.0, ..Default::default() })
        .add_startup_stage("setup", SystemStage::single(setup.system()))

        .add_startup_stage_after("setup", "object_spawn", SystemStage::serial())
        .add_startup_system_to_stage("object_spawn", spawn_background.system())
        .add_startup_system_to_stage("object_spawn", spawn_comets.system())
        .add_startup_system_to_stage("object_spawn", spawn_ship.system())

        .add_system(parralax_scrolling.system())
        .add_system(camera_follow.system())
        .add_system(ship_movement.system())
        .add_system(collision_detection.system())
        .add_system(event_handler.system())

        .add_plugins(DefaultPlugins)
    
    .run();
}

fn setup(
    commands: &mut Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default()).with(Camera);

    let ship_texture_handle = asset_server.load("textures/ship.png");
    let ship_texture_atlas = TextureAtlas::from_grid(ship_texture_handle, Vec2::new(16.0, 16.0), 1, 1);

    let comet_texture_handle = asset_server.load("textures/comet.png");
    let comet_texture_atlas = TextureAtlas::from_grid(comet_texture_handle, Vec2::new(50.0, 50.0), 1, 1);

    let background_layer1_texture_handle = asset_server.load("textures/background_layer1.png");
    let background_layer1_texture_atlas = TextureAtlas::from_grid(background_layer1_texture_handle, Vec2::new(2500.0, 2500.0), 1, 1);

    let background_layer2_texture_handle = asset_server.load("textures/background_layer2.png");
    let background_layer2_texture_atlas = TextureAtlas::from_grid(background_layer2_texture_handle, Vec2::new(2500.0, 2500.0), 1, 1);

    let background_layer3_texture_handle = asset_server.load("textures/background_layer3.png");
    let background_layer3_texture_atlas = TextureAtlas::from_grid(background_layer3_texture_handle, Vec2::new(2500.0, 2500.0), 1, 1);

    commands.insert_resource(Textures {
        ship_texture: texture_atlases.add(ship_texture_atlas),
        background_layer1_texture: texture_atlases.add(background_layer1_texture_atlas),
        background_layer2_texture: texture_atlases.add(background_layer2_texture_atlas),
        background_layer3_texture: texture_atlases.add(background_layer3_texture_atlas),
        comet_texture: texture_atlases.add(comet_texture_atlas),
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
        .with(ShipStatus { dead: false })
        .with(Rotation { angle: 0.0 });
}

fn ship_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut ship_position: Query<(&mut Transform, &mut Rotation), With<Ship>>,
) {
    for (mut transform, mut rotation) in ship_position.iter_mut() {
        let move_dir = transform.rotation * Vec3::unit_y();
        let move_speed = 15.0;
        let rotation_speed = 0.1;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            rotation.angle += rotation_speed;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            rotation.angle -= rotation_speed;
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

fn spawn_background(commands: &mut Commands, texture: Res<Textures>) {
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.background_layer1_texture.clone(),
            transform: Transform::from_scale(Vec3::splat(5.0)),
            ..Default::default()
        })
        .with(Background)
        .with(Layer { value: 1});

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.background_layer2_texture.clone(),
            transform: Transform::from_scale(Vec3::splat(7.5)),
            ..Default::default()
        })
        .with(Background)
        .with(Layer { value: 2 });
    
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.background_layer3_texture.clone(),
            transform: Transform::from_scale(Vec3::splat(12.5)),
            ..Default::default()
        })
        .with(Background)
        .with(Layer { value: 3 });
}

fn spawn_comets(
    commands: &mut Commands,
    texture: Res<Textures>,
) {
    let mut rng = rand::thread_rng();
    for _z in 1 .. 2000 { //2000 = number of comets
        let x = rng.gen_range(-25000 .. 25000);
        let y = rng.gen_range(-25000 .. 25000);
        let rotation = rng.gen_range(0.0 .. 360.0);
        let scale = rng.gen_range(1.0 .. 2.5);
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture.comet_texture.clone(),
                transform: Transform::from_scale(Vec3::splat(2.0)),
                ..Default::default()
            })
            .with(Comet)
            .with(Transform { 
                translation: Vec3 { x: x as f32, y: y as f32, z: 0.0 }, 
                rotation: Quat::from_rotation_z(rotation),
                scale: Vec3 { x: scale, y: scale, z: 0.0}
            });
    }
}

fn collision_detection(
    comets: Query<&Transform, With<Comet>>,
    mut ships: Query<(&Transform, &mut ShipStatus), With<Ship>>,
) {
    for comet in comets.iter() {
        for (ship, mut status) in ships.iter_mut() {
            if comet.translation.x < ship.translation.x + 80.0 && comet.translation.y < ship.translation.y + 80.0
            && comet.translation.x > ship.translation.x - 80.0 && comet.translation.y > ship.translation.y - 80.0 {
                status.dead = true;
            }
        }
    }
}

fn parralax_scrolling(
    ship_position: Query<&Transform, With<Ship>>,
    mut background_position: Query<(&mut Transform, &Layer), With<Background>>
) {
    for ship in ship_position.iter() {
        for (mut background, layer) in background_position.iter_mut() {
            if layer.value == 1 {
                background.translation.x = ship.translation.x / 1.25;
                background.translation.y = ship.translation.y / 1.25;
            }
            if layer.value == 2 {
                background.translation.x = ship.translation.x / 1.5;
                background.translation.y = ship.translation.y / 1.5;
            }

            if layer.value == 3 {
                background.translation.x = ship.translation.x / 2.0;
                background.translation.y = ship.translation.y / 2.0;
            }
        }
    }
}

fn camera_follow(
    ship_position: Query<&Transform, With<Ship>>,
    mut camera_position: Query<&mut Transform, With<Camera>>
) {
    for ship in ship_position.iter() {
        let ship_dir = ship.rotation * Vec3::unit_y();
        for mut camera in camera_position.iter_mut() {
            camera.translation = ship.translation + ship_dir * 200.0;
            camera.rotation = ship.rotation;
        }
    }
}

fn event_handler(
    ship: Query<&ShipStatus, With<Ship>>,
) {
    for status in ship.iter() {
        if status.dead == true {
            panic!("Dead!");
        }
    }
}

