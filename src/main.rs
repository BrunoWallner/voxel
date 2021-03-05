use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, Input};

struct Materials {
    ship_material: Handle<ColorMaterial>
}

#[derive(Default, Copy, Clone, PartialEq)]
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
        .add_resource(WindowDescriptor { title: "Space Shooter".to_string(), width:1200.0, height: 800.0, ..Default::default() })
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_ship.system()))
        .add_system(ship_movement.system())
        .add_plugins(DefaultPlugins)
    
    .run();
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(Materials {
        ship_material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
    });
}

fn spawn_ship(commands: &mut Commands, materials: Res<Materials>) {
    commands
        .spawn(SpriteBundle {
            material: materials.ship_material.clone(),
            sprite: Sprite::new(Vec2::new(50.0, 50.0)),
            ..Default::default()
        })
        .with(Ship)
        .with(Position { x: 5.0, y: 3.0 })
        .with(Size { width: 18.0, height: 25.0 })
        .with(Speed { x: 10.0, y: 10.0 });
}

fn ship_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut ship_position: Query<(&mut Position, &mut Transform, &Speed), With<Ship>>,
    mut ship_size: Query<(&Size, &mut Sprite), With<Ship>>,
    //mut ship_transform: Query<&mut Transform, With<Ship>>,
) {
    for (mut position, mut transform, speed) in ship_position.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            position.x -= speed.x;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            position.x += speed.x;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            position.y -= speed.y;
        }
        if keyboard_input.pressed(KeyCode::Up) {
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


