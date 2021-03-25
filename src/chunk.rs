use bevy::prelude::*;
use bevy::render::pipeline::PrimitiveTopology;

use noise::NoiseFn;
use noise::OpenSimplex;
use noise::Seedable;

pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub index: [[[u8; 16]; 256]; 16],
    pub loaded: bool,
    pub should_load: bool,
}
impl Chunk {
    //creates a new empty chunk filled with air
    pub fn new(x: i32, z: i32) -> Self {
        Chunk {x: x, z: z, index: generate_chunk_index(100, x, z), loaded: false, should_load: true}
    }
}

pub fn generate_chunk_index(
    seed: u32,
    chunk_x: i32,
    chunk_z: i32,
) -> [[[u8; 16]; 256]; 16] {

    let chunk_size: usize = 16;
    let noise = OpenSimplex::new();
    noise.set_seed(seed);

    let mut index: [[[u8; 16]; 256]; 16] = [[[0u8; 16]; 256]; 16];

    for x in 0 .. chunk_size {
        for z in 0 .. chunk_size {
            let y = (noise.get([
                ( (x as i32 + chunk_x * 16 as i32) as f32 / 20. ) as f64, 
                ( (z as i32 + chunk_z * 16 as i32) as f32 / 20. ) as f64,
            ]) * 20. + 16.0) as usize;

            index[x][y][z] = 1;

            for i in 0..y-1 {
                index[x][i][z] = 2;
            }
        }
    }
    index
}

use crate::Materials;
use crate::Camera;

pub fn chunk_loader(
    camera: Query<&Transform, With<Camera>>,
    mut chunk: Query<&mut Chunk, With<Chunk>>,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Materials>,
) {
    let render_distance: i32 = 10;

    //creates empty chunk if needed
    for x in -render_distance..render_distance {
        for z in -render_distance..render_distance {
            let mut camera_chunk: [i32; 2] = [0, 0];
            let mut is_empty_chunk: bool = false;

            for camera in camera.iter() {
                camera_chunk = [(camera.translation.x / 16.0 + x as f32) as i32, (camera.translation.z / 16.0 + z as f32) as i32]
            }
            for chunk in chunk.iter_mut() {
                if chunk.x == camera_chunk[0]
                && chunk.z == camera_chunk[1] {
                    is_empty_chunk = true;
                }
            }

            if !is_empty_chunk {
                commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane{ size: 1.0 })),
                    material: materials.blocks[1].clone(),
                    transform: Transform::from_translation(Vec3::new((camera_chunk[0]*16 + 8) as f32, 0.0, (camera_chunk[1]*16 + 8) as f32)),
                    ..Default::default()
                })
                .with(Chunk::new(camera_chunk[0], camera_chunk[1]));
            }
        }
    }   
}

use bevy::render::mesh::Indices;

pub fn spawn_chunk(
    commands: &mut Commands,
    materials: Res<Materials>,
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

                            //creates vertices
                            for i in 0..2 {
                                positions.push([x as f32, (y + i) as f32, z as f32]);
                                normals.push([x as f32, y as f32, z as f32]);
                                uvs.push([0.0, 0.0]);

                                positions.push([(x + 1) as f32, (y + i) as f32, z as f32]);
                                normals.push([x as f32, y as f32, z as f32]);
                                uvs.push([0.0, 0.0]);

                                positions.push([(x + 1) as f32, (y + i) as f32, (z + 1) as f32]);
                                normals.push([x as f32, y as f32, z as f32]);
                                uvs.push([0.0, 0.0]);

                                positions.push([x as f32, (y + i) as f32, (z + 1) as f32]);
                                normals.push([x as f32, y as f32, z as f32]);
                                uvs.push([0.0, 0.0]);
                            }

                            //creates indices
                        
                            //below plane
                            let below = create_indices("below", positions.len() as u32);
                            if y >= 1 {
                                if chunk.index[x][y-1][z] == 0 {
                                    for i in 0..6 {
                                        indices.push(below[i]);
                                    }
                                }
                            } else {
                                for i in 0..6 {
                                    indices.push(below[i]);
                                }
                            }

                            //above plane
                            let above = create_indices("above", positions.len() as u32);
                            if y <= 254 {
                                if chunk.index[x][y+1][z] == 0 {
                                    for i in 0..6 {
                                        indices.push(above[i]);
                                    }
                                }
                            } else {
                                for i in 0..6 {
                                    indices.push(above[i]);
                                }
                            }

                            //left plane
                            let left = create_indices("left", positions.len() as u32);
                            if z >= 1 {
                                if chunk.index[x][y][z-1] == 0 {
                                    for i in 0..6 {
                                        indices.push(left[i]);
                                    }
                                }
                            } else {
                                for i in 0..6 {
                                    indices.push(left[i]);
                                }
                            }

                            //right plane
                            let right = create_indices("right", positions.len() as u32);
                            if z <= 14 {
                                if chunk.index[x][y][z+1] == 0 {
                                    for i in 0..6 {
                                        indices.push(right[i]);
                                    }
                                }
                            } else {
                                for i in 0..6 {
                                    indices.push(right[i]);
                                }
                            }
                            
                            //front plane
                            let front = create_indices("front", positions.len() as u32);
                            if x >= 1 {
                               if chunk.index[x-1][y][z] == 0 {
                                for i in 0..6 {
                                    indices.push(front[i]);
                                }
                                } 
                            } else {
                                for i in 0..6 {
                                    indices.push(front[i]);
                                }
                            }
                            
                            //back plane
                            let back = create_indices("back", positions.len() as u32);
                            if x <= 14 {
                                if chunk.index[x+1][y][z] == 0 {
                                    for i in 0..6 {
                                        indices.push(back[i]);
                                    }
                                }
                            } else {
                                for i in 0..6 {
                                    indices.push(back[i]);
                                }
                            }
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
                    material: materials.blocks[1].clone(),
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

fn create_indices(
    side: &str,
    position_len: u32,
) -> [u32; 6] {
    let mut indices: [u32; 6] = [0, 0, 0, 0, 0, 0];

    if side == "below" {
        indices[0] = (position_len - 8 + 0) as u32;
        indices[1] = (position_len - 8 + 1) as u32;
        indices[2] = (position_len - 8 + 3) as u32;

        indices[3] = (position_len - 8 + 2) as u32;
        indices[4] = (position_len - 8 + 3) as u32;
        indices[5] = (position_len - 8 + 1) as u32;
    }

    if side == "above" {
        indices[0] = (position_len - 8 + 5) as u32;
        indices[1] = (position_len - 8 + 4) as u32;
        indices[2] = (position_len - 8 + 6) as u32;

        indices[3] = (position_len - 8 + 4) as u32;
        indices[4] = (position_len - 8 + 7) as u32;
        indices[5] = (position_len - 8 + 6) as u32;
    }

    if side == "left" {
        indices[0] = (position_len - 8 + 1) as u32;
        indices[1] = (position_len - 8 + 0) as u32;
        indices[2] = (position_len - 8 + 5) as u32;

        indices[3] = (position_len - 8 + 0) as u32;
        indices[4] = (position_len - 8 + 4) as u32;
        indices[5] = (position_len - 8 + 5) as u32;
    }

    if side == "right" {
        indices[0] = (position_len - 8 + 3) as u32;
        indices[1] = (position_len - 8 + 2) as u32;
        indices[2] = (position_len - 8 + 6) as u32;

        indices[3] = (position_len - 8 + 7) as u32;
        indices[4] = (position_len - 8 + 3) as u32;
        indices[5] = (position_len - 8 + 6) as u32;        
    }

    if side == "front" {
        indices[0] = (position_len - 8 + 0) as u32;
        indices[1] = (position_len - 8 + 3) as u32;
        indices[2] = (position_len - 8 + 4) as u32;

        indices[3] = (position_len - 8 + 4) as u32;
        indices[4] = (position_len - 8 + 3) as u32;
        indices[5] = (position_len - 8 + 7) as u32;        
    }

    if side == "back" {
        indices[0] = (position_len - 8 + 2) as u32;
        indices[1] = (position_len - 8 + 1) as u32;
        indices[2] = (position_len - 8 + 5) as u32;

        indices[3] = (position_len - 8 + 2) as u32;
        indices[4] = (position_len - 8 + 5) as u32;
        indices[5] = (position_len - 8 + 6) as u32;
    }
    indices
}