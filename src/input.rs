use bevy::prelude::*;
use crate::Camera;

pub struct Builder {
    x: i32,
    y: i32,
    z: i32,
}
impl Builder {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Builder {x: x, y: y, z: z}
    }

    pub fn chpos(&mut self, direction: [i32; 3]) {
        self.x += direction[0];
        self.y += direction[1];
        self.z += direction[2];
    }

    pub fn get_position(&self) -> [i32; 3] {
        [self.x, self.y ,self.z]
    }
}

pub fn build(
    mut world: Query<&mut crate::chunk::World, With<crate::chunk::World>>,
    builder: Query<&Builder, With<Builder>>,
    mut chunk_mesh: Query<(Entity, &mut crate::chunk::ChunkMesh), With<crate::chunk::ChunkMesh>>,
    input: Res<Input<KeyCode>>,
    commands: &mut Commands,
    materials: Res<crate::Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut builder_position: [i32; 3] = [0, 0, 0];

    for builder in builder.iter() {
        builder_position = builder.get_position();
    }

    let mut builder_chunk_position: [i32; 3] = 
    
    [builder_position[0] / 32, builder_position[1] / 32, builder_position[2] / 32];
    // important
    if builder_position[0] < 0 {
        builder_chunk_position[0]-=1;
    }
    if builder_position[1] < 0 {
        builder_chunk_position[1]-=1;
    }
    if builder_position[2] < 0 {
        builder_chunk_position[2]-=1;
    }

    let mut chunk_position: usize = 0;
    for mut world in world.iter_mut() {
        chunk_position = 
            crate::chunk::get_chunk_index( builder_chunk_position, &mut world ); 
    }

    let mut blocks: [usize; 3] = [0, 0, 0];

    for i in 0..3 {
        if builder_chunk_position[i] > 0 {
            blocks[i] = ( builder_position[i] - builder_chunk_position[i] *32 ) as usize
        }
        if builder_chunk_position[i] < 0 {
            blocks[i] = ( builder_position[i] - builder_chunk_position[i] *32 ) as usize
        }
        if builder_chunk_position[i] == 0 {
            blocks[i] = builder_position[i] as usize;
        }
    }

    if input.pressed(KeyCode::E) {
        for mut world in world.iter_mut() {

            // places block in chunk index
            world
                .chunk_index[chunk_position]
                .index[blocks[0]][blocks[1]][blocks[2]] = 2;

            // shows debug infos
            print!("\x1B[2J\x1B[1;1H");
            println!("-------------DEBUG-------------");
            println!("destroyed block at: {} {} {}", blocks[0], blocks[1], blocks[2]);
            println!("in chunk:           {} {} {}", builder_chunk_position[0], builder_chunk_position[1], builder_chunk_position[2]);
            println!("builder psition:    {} {} {}", builder_position[0], builder_position[1], builder_position[2]);
        }

        // replaces ChunkMesh
        for (entity, chunk_mesh) in chunk_mesh.iter_mut() {
            if chunk_mesh.x == builder_chunk_position[0]
            && chunk_mesh.y == builder_chunk_position[1]
            && chunk_mesh.z == builder_chunk_position[2]
            {
                commands.despawn(entity);

                for mut world in world.iter_mut() {
                    let mesh = 
                        crate::chunk::create_chunk_mesh(
                            crate::chunk::get_chunk_index(builder_chunk_position, &mut world),
                            &mut world
                        );

                    commands
                        .spawn(PbrBundle {
                            mesh: meshes.add(mesh),
                            material: materials.blocks.clone(),
                            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                                Vec3::splat(1.0),
                                Quat::from_rotation_x(0.0),
                                Vec3::new((builder_chunk_position[0] * 32) as f32, (builder_chunk_position[1] * 32) as f32, (builder_chunk_position[2] * 32) as f32),
                            )),
                            ..Default::default()
                        })
                        .with(crate::chunk::ChunkMesh::new(builder_chunk_position[0], builder_chunk_position[1], builder_chunk_position[2]));
                }
            }
        }
    }
}

pub fn builder_movement(
    input: Res<Input<KeyCode>>,
    mut builder: Query<&mut Builder, With<Builder>>,
) {
    for mut builder in builder.iter_mut() {

        let speed: i32 = 1;

        if input.just_pressed(KeyCode::Left) {
            builder.chpos( [0, 0, -speed] );
        }
        if input.just_pressed(KeyCode::Up) {
            builder.chpos( [speed, 0, 0] );
        }
        if input.just_pressed(KeyCode::Right) {
            builder.chpos( [0, 0, speed] );
        }
        if input.just_pressed(KeyCode::Down) {
            builder.chpos( [-speed, 0, 0] );
        }
    }
}

pub fn movement(
    mut camera: Query<&mut Transform, With<Camera>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for mut camera in camera.iter_mut() {
        let speed: f32 = 50.0 * time.delta_seconds(); 
        if input.pressed(KeyCode::W) {
            camera.translation.z+=speed;
            camera.translation.x+=speed;
        }
        if input.pressed(KeyCode::A) {
            camera.translation.z-=speed / 2.0;
            camera.translation.x+=speed / 2.0;
        }
        if input.pressed(KeyCode::S) {
            camera.translation.z-=speed;
            camera.translation.x-=speed;
        }
        if input.pressed(KeyCode::D) {
            camera.translation.z+=speed / 2.0;
            camera.translation.x-=speed / 2.0;
        }
    }
}