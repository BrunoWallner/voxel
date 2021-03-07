use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, Input};

struct Textures {
    ship_texture: Handle<TextureAtlas>,
    background_texture: Handle<TextureAtlas>,
    background_low_texture: Handle<TextureAtlas>,
    background_med_texture: Handle<TextureAtlas>,
    comet_texture: Handle<TextureAtlas>,
}

struct Rotation {
    speed: f32,
    angle: f32,
}

struct ShipEvent {
    pub status: String,
}

struct Ship;
struct Background;
struct Camera;
struct Dummy;
struct Comet;

pub fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(WindowDescriptor { title: "Space Shooter".to_string(), width:1200.0, height: 800.0, ..Default::default() })
        .add_startup_stage("setup", SystemStage::single(setup.system()))

        .add_startup_stage_after("setup", "object_spawn", SystemStage::serial())
        .add_startup_system_to_stage("object_spawn", spawn_background.system())
        .add_startup_system_to_stage("object_spawn", spawn_ship.system())
        .add_startup_system_to_stage("object_spawn", spawn_comets.system())

        .add_event::<ShipEvent>()

        .add_system(parralax_scrolling.system())
        .add_system(camera_follow.system())
        .add_system(ship_movement.system())
        .add_system(collision_detection.system())

        .add_system(despawn_dummys.system())

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

    let background_texture_handle = asset_server.load("textures/background.png");
    let background_texture_atlas = TextureAtlas::from_grid(background_texture_handle, Vec2::new(1000.0, 1000.0), 1, 1);

    let background_low_texture_handle = asset_server.load("textures/background_low.png");
    let background_low_texture_atlas = TextureAtlas::from_grid(background_low_texture_handle, Vec2::new(1000.0, 1000.0), 1, 1);

    let background_med_texture_handle = asset_server.load("textures/background_med.png");
    let background_med_texture_atlas = TextureAtlas::from_grid(background_med_texture_handle, Vec2::new(1000.0, 1000.0), 1, 1);

    commands.insert_resource(Textures {
        ship_texture: texture_atlases.add(ship_texture_atlas),
        background_texture: texture_atlases.add(background_texture_atlas),
        background_low_texture: texture_atlases.add(background_low_texture_atlas),
        background_med_texture: texture_atlases.add(background_med_texture_atlas),
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

fn spawn_background(commands: &mut Commands, texture: Res<Textures>) {
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.background_low_texture.clone(),
            transform: Transform::from_scale(Vec3::splat(10.0)),
            ..Default::default()
        })
        .with(Dummy)
        .with(Background);

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.background_med_texture.clone(),
            transform: Transform::from_scale(Vec3::splat(10.0)),
            ..Default::default()
        })
        .with(Dummy)
        .with(Background);

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.background_texture.clone(),
            transform: Transform::from_scale(Vec3::splat(10.0)),
            ..Default::default()
        })
        .with(Background);
}

fn spawn_comets(
    commands: &mut Commands,
    texture: Res<Textures>,
) {
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.comet_texture.clone(),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..Default::default()
        })
        .with(Comet);
}

fn collision_detection(
    comets: Query<&Transform, With<Comet>>,
    ships: Query<&Transform, With<Ship>>,
    mut events: ResMut<Events<ShipEvent>>,
) {
    for comet in comets.iter() {
        for ship in ships.iter() {
            if comet.translation.x < ship.translation.x + 0.5 && comet.translation.y < ship.translation.y + 0.5 
            && comet.translation.x > ship.translation.x - 0.5 && comet.translation.y > ship.translation.y - 0.5 {
                events.send(ShipEvent {
                    status: "dead".to_string(),
                })
            }
        }
    }
}

fn parralax_scrolling(
    ship_position: Query<&Transform, With<Ship>>,
    mut background_position: Query<&mut Transform, With<Background>>
) {
    for ship in ship_position.iter() {
        for mut background in background_position.iter_mut() {
            background.translation.x = ship.translation.x / 1.25;
            background.translation.y = ship.translation.y / 1.25;
        }
    }
}

fn camera_follow(
    ship_position: Query<&Transform, With<Ship>>,
    mut camera_position: Query<&mut Transform, With<Camera>>
) {
    for ship in ship_position.iter() {
        for mut camera in camera_position.iter_mut() {
            camera.translation = ship.translation;
        }
    }
}

fn despawn_dummys(
    commands: &mut Commands,
    dummys: Query<&Entity, With<Dummy>>
) {
    for dummy in dummys.iter() {
        commands.despawn(*dummy);
    }
}

