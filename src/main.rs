use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, Input};

struct Textures {
    ship_texture: Handle<TextureAtlas>
}

struct Position {
    x: f32,
    y: f32,
}

struct Size {
    width: f32,
    height: f32,
}

struct Speed {
    x: f32,
    y: f32,
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
        .with(Position { x: 5.0, y: 3.0 })
        .with(Size { width: 16.0, height: 16.0 })
        .with(Speed { x: 10.0, y: 10.0 });
}

fn ship_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut ship_position: Query<(&mut Position, &mut Transform, &Speed), With<Ship>>,
    mut ship_size: Query<(&Size, &mut Sprite), With<Ship>>,
    //mut ship_transform: Query<&mut Transform, With<Ship>>,
) {
    for (mut position, mut transform, speed) in ship_position.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            position.x -= speed.x;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            position.x += speed.x;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            position.y -= speed.y;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            position.y += speed.y;
        }
        //applies transformation
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
    for (size, mut sprite) in ship_size.iter_mut() {
        sprite.size = Vec2::new(size.width, size.height);
    }
}


