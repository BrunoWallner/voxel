use bevy::prelude::*;
use bevy::render::pipeline::PrimitiveTopology;

use noise::NoiseFn;
use noise::OpenSimplex;
use noise::Seedable;

use rand::Rng;
use rand::thread_rng;

use crate::Materials;
use crate::Chunk;
use crate::Camera;
use crate::Seed;


pub fn create_chunk(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Materials>,
) {
    let chunk_size = 16;

    for chunk_x in -10 .. 10 { 
        for chunk_z in -10 .. 10 {

            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane{ size: 1.0 })),
                    material: materials.blocks[1].clone(),
                    transform: Transform::from_translation(Vec3::new((chunk_x * chunk_size) as f32, 0.0, (chunk_z * chunk_size) as f32)),
                    ..Default::default()
                })
                .with(Chunk::new(chunk_x, chunk_z));
            }
        }
}

pub fn chunk_loader(
    camera: Query<&Transform, With<Camera>>,
    mut chunk: Query<&mut Chunk, With<Chunk>>,
) {
    let render_distance: i32 = 3;
    for camera in camera.iter() {
        for mut chunk in chunk.iter_mut() {
            if !chunk.loaded
            && camera.translation.x > (chunk.x*16 - 16*render_distance) as f32
            && camera.translation.x < (chunk.x*16 + 16*render_distance) as f32
            && camera.translation.z > (chunk.z*16 - 16*render_distance) as f32
            && camera.translation.z < (chunk.z*16 + 16*render_distance) as f32
            {
                chunk.should_load = true;
            }
            else {
                chunk.should_load = false;
            }
        }
    } 
}

pub fn generate_chunk(
    mut chunks: Query<&mut Chunk, With<Chunk>>,
    seed: Res<Seed>,
    time: Res<Time>,
) {
    let start_time = time.time_since_startup();
    let mut chunk_counter: u32 = 0;

    let chunk_size: usize = 16;
    let noise = OpenSimplex::new();
    noise.set_seed(seed.value);

    for mut chunk in chunks.iter_mut() {
        for x in 0 .. chunk_size {
            for z in 0 .. chunk_size {
                
                let rng = thread_rng().gen_range(1 .. 10000);

                let y = (noise.get([
                    ( (x as i32 + chunk.x * chunk_size as i32) as f32 / 20. ) as f64, 
                    ( (z as i32 + chunk.z * chunk_size as i32) as f32 / 20. ) as f64,
                ]) * 20. + 16.0) as usize;

                /*
                for i in 0 .. y - 4 {
                    chunk.index[x][i][z] = 2;
                }
                for i in y - 4 .. y  {
                    chunk.index[x][i][z] = 1;
                }
                */
                chunk.index[x][y][z] = 1;

                //generates random stones on surface
                if rng & 5000 == 0 {
                    chunk.index[x][y][z] = 2;
                }
            }
        }
        chunk_counter+=1;
    }
    let end_time = time.time_since_startup() - start_time;
    println!("Generated {} chunks in {:?}", chunk_counter, end_time);
}

use bevy::render::mesh::Indices;

pub fn spawn_chunk(
    commands: &mut Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunks: Query<&mut Chunk, With<Chunk>>,
) {
    for mut chunk in chunks.iter_mut() {

        if chunk.should_load {

            let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

            let mut positions: Vec<[f32; 3]> = Vec::new();
            let mut normals: Vec<[f32; 3]> = Vec::new();
            let mut uvs: Vec<[f32; 2]> = Vec::new();

            let mut indices: Vec<u32> = Vec::new();
        
            for x in 0..16 {
                for y in 0..256 {
                    for z in 0..16 {
                        if chunk.index[x][y][z] != 0 {
                            for i in 0..2 {
                                positions.push([x as f32, (y + i) as f32, z as f32]);
                                normals.push([0., 0., 0.]);
                                uvs.push([0., 0.]);

                                positions.push([(x + 1) as f32, (y + i) as f32, z as f32]);
                                normals.push([0., 0., 0.]);
                                uvs.push([0., 0.]);

                                positions.push([(x + 1) as f32, (y + i) as f32, (z + 1) as f32]);
                                normals.push([0., 0., 0.]);
                                uvs.push([0., 0.]);

                                positions.push([x as f32, (y + i) as f32, (z + 1) as f32]);
                                normals.push([0., 0., 0.]);
                                uvs.push([0., 0.]);
                            }

                            //creates indices
                            indices.push((positions.len() - 8 + 1) as u32);
                            indices.push((positions.len() - 8 + 0) as u32);
                            indices.push((positions.len() - 8 + 2) as u32);

                            indices.push((positions.len() - 8 + 0) as u32);
                            indices.push((positions.len() - 8 + 3) as u32);
                            indices.push((positions.len() - 8 + 2) as u32);
                        }
                    }
                }
            }


            mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

            mesh.set_indices(Some(Indices::U32(indices)));


            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(Color::rgb(0.2, 1.0, 0.5).into()),
                    transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                        Vec3::splat(1.0),
                        Quat::from_rotation_x(0.0),
                        Vec3::new((chunk.x * 16) as f32, 0.0, (chunk.z * 16) as f32),
                    )),
                    ..Default::default()
                });

            chunk.should_load = false;
            chunk.loaded = true;
        }       
    }
}