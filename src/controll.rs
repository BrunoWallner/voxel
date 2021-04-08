use bevy::prelude::*;

pub struct Builder {
    x: f32,
    y: f32,
    z: f32,
    distance: f32,
}
impl Builder {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Builder {x: x, y: y, z: z, distance: 5.0}
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
    input: Res<Input<MouseButton>>,
    mut commands: Commands,
    materials: Res<crate::Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut builder_position: [i32; 3] = [0, 0, 0];
    let mut builder_raw_position: [f32; 3] = [0.0, 0.0, 0.0];

    for builder in builder.iter() {
        let old_pos = builder.get_position();
        builder_position = [(old_pos[0] - 1.0) as i32, (old_pos[1] - 0.25) as i32, (old_pos[2] - 1.0) as i32];

        // updates position
        builder_raw_position = [old_pos[0], old_pos[1], old_pos[2]];
    }

    let builder_chunk_position: [i32; 3] = crate::chunk::get_chunk_coordinates_from_position(builder_position);

    let mut chunk_position: usize = 0;

    for mut world in world.iter_mut() {
        chunk_position = 
            crate::chunk::get_chunk_index( builder_chunk_position, &mut world ); 
    }

    let mut blocks: [usize; 3] = [0, 0, 0];

    for i in 0..3 {
        blocks[i] = ( (builder_position[i]) - builder_chunk_position[i] *32 ) as usize
    }

    for mut builder_indicator in builder_indicator.iter_mut() {
        builder_indicator.translation.x = builder_raw_position[0];
        builder_indicator.translation.y = builder_raw_position[1];
        builder_indicator.translation.z = builder_raw_position[2];
    }


    let mut edited: bool = false;

    // places block
    if input.pressed(MouseButton::Right) {
        for mut world in world.iter_mut() {

            // places block in chunk index
            world
                .chunk_index[chunk_position]
                .index[blocks[0]][blocks[1]][blocks[2]] = 3;
        }
        edited = true;
    }

    // destroys block
    if input.pressed(MouseButton::Left) {
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
) {
    for mut builder in builder.iter_mut() {
        if input.just_pressed(KeyCode::Up) {
            builder.distance += 1.0;
        }
        if input.just_pressed(KeyCode::Down) {
            builder.distance -= 1.0;
        }
    }
}

pub fn movement(
    camera: Query<&Transform, With<crate::Camera>>,
    mut builder: Query<&mut Builder, With<Builder>>,
) {
    for mut builder in builder.iter_mut() {
        for camera in camera.iter() {
            let local_z = camera.local_z();
            let forward = -Vec3::new(local_z.x, local_z.y, local_z.z);

            builder.x = camera.translation.x + forward[0] * builder.distance;
            builder.y = camera.translation.y + forward[1] * builder.distance;
            builder.z = camera.translation.z + forward[2] * builder.distance;
        }
    }
}