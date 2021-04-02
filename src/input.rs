use bevy::prelude::*;
use crate::Camera;

pub fn place(
    mut world: Query<&mut crate::chunk::World, With<crate::chunk::World>>,
    camera: Query<&Transform, With<Camera>>,
    mut chunk_mesh: Query<(Entity, &mut crate::chunk::ChunkMesh), With<crate::chunk::ChunkMesh>>,
    input: Res<Input<KeyCode>>,
    commands: &mut Commands,

    materials: Res<crate::Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut camera_position: [f32; 3] = [0., 0., 0.];

    for camera in camera.iter() {
        camera_position = [camera.translation.x, camera.translation.y, camera.translation.z];
    }

    let mut camera_chunk_position: [i32; 3] = 
        [(camera_position[0] / 32.) as i32, (camera_position[1] / 32.) as i32, (camera_position[2] / 32.) as i32];

    // important
    if camera_position[0] < 0. {
        camera_chunk_position[0]-=1;
    }
    if camera_position[1] < 0. {
        camera_chunk_position[1]-=1;
    }
    if camera_position[2] < 0. {
        camera_chunk_position[2]-=1;
    }

    let mut chunk_position: usize = 0;
    for mut world in world.iter_mut() {
        chunk_position = 
            crate::chunk::get_chunk_index( camera_chunk_position, &mut world ); 
    }

    let mut blocks: [usize; 3] = [0, 0, 0];

    for i in 0..3 {
        if camera_chunk_position[i] > 0 {
            blocks[i] = ( camera_position[i] - camera_chunk_position[i] as f32 *32. ) as usize
        }
        if camera_chunk_position[i] < 0 {
            blocks[i] = ( camera_position[i] - camera_chunk_position[i] as f32 *32. ) as usize
        }
        if camera_chunk_position[i] == 0 {
            blocks[i] = camera_position[i] as usize;
        }
    }

    if input.just_pressed(KeyCode::E) {
        for mut world in world.iter_mut() {

            world
                .chunk_index[chunk_position]
                .index[blocks[0]][blocks[1]][blocks[2]] = 2;

            print!("\x1B[2J\x1B[1;1H");
            println!("-------------DEBUG-------------");
            println!("destroyed block at: {} {} {}", blocks[0], blocks[1], blocks[2]);
            println!("in chunk:           {} {} {} !", camera_chunk_position[0], camera_chunk_position[1], camera_chunk_position[2]);
        }

        //replaces ChunkMesh
        for (entity, chunk_mesh) in chunk_mesh.iter_mut() {
            if chunk_mesh.x == camera_chunk_position[0]
            && chunk_mesh.y == camera_chunk_position[1]
            && chunk_mesh.z == camera_chunk_position[2]
            {
                commands.despawn(entity);

                for mut world in world.iter_mut() {
                    let mesh = 
                        crate::chunk::create_chunk_mesh(
                            crate::chunk::get_chunk_index(camera_chunk_position, &mut world),
                            &mut world
                        );

                    commands
                        .spawn(PbrBundle {
                            mesh: meshes.add(mesh),
                            material: materials.blocks.clone(),
                            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                                Vec3::splat(1.0),
                                Quat::from_rotation_x(0.0),
                                Vec3::new((camera_chunk_position[0] * 32) as f32, (camera_chunk_position[1] * 32) as f32, (camera_chunk_position[2] * 32) as f32),
                            )),
                            ..Default::default()
                        })
                        .with(crate::chunk::ChunkMesh::new(camera_chunk_position[0], camera_chunk_position[1], camera_chunk_position[2]));
                }
            }
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
            camera.translation.z-=speed;
            camera.translation.x-=speed;
        }
        if input.pressed(KeyCode::A) {
            camera.translation.z+=speed / 2.0;
            camera.translation.x-=speed / 2.0;
        }
        if input.pressed(KeyCode::S) {
            camera.translation.z+=speed;
            camera.translation.x+=speed;
        }
        if input.pressed(KeyCode::D) {
            camera.translation.z-=speed / 2.0;
            camera.translation.x+=speed / 2.0;
        }
    }
}