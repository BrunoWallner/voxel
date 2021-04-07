use bevy::prelude::*;
use crate::Camera;

pub struct Builder {
    x: f32,
    y: f32,
    z: f32,
}
impl Builder {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Builder {x: x, y: y, z: z}
    }

    pub fn chpos(&mut self, direction: [f32; 3]) {
        self.x += direction[0];
        self.y += direction[1];
        self.z += direction[2];
    }

    pub fn get_position(&self) -> [f32; 3] {
        [self.x, self.y ,self.z]
    }
}

pub struct BuilderIndicator;

pub fn build(
    mut world: Query<&mut crate::chunk::World, With<crate::chunk::World>>,
    builder: Query<&Builder, With<Builder>>,
    mut builder_indicator: Query<&mut Transform, With<BuilderIndicator>>,
    mut chunk_mesh: Query<(Entity, &mut crate::chunk::ChunkMesh), With<crate::chunk::ChunkMesh>>,
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    materials: Res<crate::Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut builder_position: [i32; 3] = [0, 0, 0];

    for builder in builder.iter() {
        let old_pos = builder.get_position();
        builder_position = [old_pos[0] as i32, old_pos[1] as i32, old_pos[2] as i32];
    }

    let builder_chunk_position: [i32; 3] = crate::chunk::get_chunk_coordinates_from_position(builder_position);

    let mut chunk_position: usize = 0;

    let mut x_chunk_position: [usize; 2] = [0, 0];
    let mut y_chunk_position: [usize; 2] = [0, 0];
    let mut z_chunk_position: [usize; 2] = [0, 0];

    for mut world in world.iter_mut() {
        chunk_position = 
            crate::chunk::get_chunk_index( builder_chunk_position, &mut world ); 

        // neighbour chunk positions
        x_chunk_position[0] = 
            crate::chunk::get_chunk_index( [builder_chunk_position[0]-1, builder_chunk_position[1], builder_chunk_position[2]], &mut world );
        x_chunk_position[1] = 
            crate::chunk::get_chunk_index( [builder_chunk_position[0]+1, builder_chunk_position[1], builder_chunk_position[2]], &mut world );

        y_chunk_position[0] = 
            crate::chunk::get_chunk_index( [builder_chunk_position[0], builder_chunk_position[1]-1, builder_chunk_position[2]], &mut world );
        y_chunk_position[1] = 
            crate::chunk::get_chunk_index( [builder_chunk_position[0], builder_chunk_position[1]+1, builder_chunk_position[2]], &mut world );

        z_chunk_position[0] = 
            crate::chunk::get_chunk_index( [builder_chunk_position[0], builder_chunk_position[1], builder_chunk_position[2]-1], &mut world );
        z_chunk_position[1] = 
            crate::chunk::get_chunk_index( [builder_chunk_position[0], builder_chunk_position[1], builder_chunk_position[2]+1], &mut world );
    }

    let mut blocks: [usize; 3] = [0, 0, 0];

    for i in 0..3 {
        blocks[i] = ( (builder_position[i]) - builder_chunk_position[i] *32 ) as usize
    }

    // update position of builder_indicator
    for mut builder_indicator in builder_indicator.iter_mut() {
        builder_indicator.translation.x = builder_position[0] as f32 + 0.5;
        builder_indicator.translation.y = builder_position[1] as f32 + 0.5;
        builder_indicator.translation.z = builder_position[2] as f32 + 0.5;
    }

    let mut edited: bool = false;

    // places block
    if input.pressed(KeyCode::E) {
        for mut world in world.iter_mut() {

            // places block in chunk index
            world
                .chunk_index[chunk_position]
                .index[blocks[0]][blocks[1]][blocks[2]] = 3;
        }
        edited = true;
    }

    // destroys block
    if input.pressed(KeyCode::Q) {
        for mut world in world.iter_mut() {

            // places block in chunk index         
            world
                .chunk_index[chunk_position]
                .index[blocks[0]][blocks[1]][blocks[2]] = 0;
            
        }
        edited = true;
    }

    // replaces ChunkMesh
    if edited {
        for (entity, chunk_mesh) in chunk_mesh.iter_mut() {
            if chunk_mesh.x == builder_chunk_position[0]
            && chunk_mesh.y == builder_chunk_position[1]
            && chunk_mesh.z == builder_chunk_position[2]
            {
                commands.entity(entity).despawn();

                for mut world in world.iter_mut() {
                    let mesh = 
                        crate::chunk::create_chunk_mesh(
                            crate::chunk::get_chunk_index(builder_chunk_position, &mut world),
                            &mut world
                        );

                    commands
                        .spawn_bundle(PbrBundle {
                            mesh: meshes.add(mesh),
                            material: materials.blocks.clone(),
                            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                                Vec3::splat(1.0),
                                Quat::from_rotation_x(0.0),
                                Vec3::new((builder_chunk_position[0] * 32) as f32, (builder_chunk_position[1] * 32) as f32, (builder_chunk_position[2] * 32) as f32),
                            )),
                            ..Default::default()
                        })
                        .insert(crate::chunk::ChunkMesh::new(builder_chunk_position[0], builder_chunk_position[1], builder_chunk_position[2]));
                }
            }
        } 
    }
    
}


pub fn builder_movement(
    input: Res<Input<KeyCode>>,
    mut builder: Query<&mut Builder, With<Builder>>,
    time: Res<Time>,
) {
    for mut builder in builder.iter_mut() {

        let speed: f32 = 10.0 * time.delta_seconds();

        if input.pressed(KeyCode::Left) {
            builder.chpos( [0.0, 0.0, -speed] );
        }
        if input.pressed(KeyCode::Up) {
            builder.chpos( [speed, 0.0, 0.0] );
        }
        if input.pressed(KeyCode::Right) {
            builder.chpos( [0.0, 0.0, speed] );
        }
        if input.pressed(KeyCode::Down) {
            builder.chpos( [-speed, 0.0, 0.0] );
        }

        if input.pressed(KeyCode::Numpad0) {
            builder.chpos( [0.0, -speed, 0.0] );
        }
        if input.pressed(KeyCode::RControl) {
            builder.chpos( [0.0, speed, 0.0] );
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