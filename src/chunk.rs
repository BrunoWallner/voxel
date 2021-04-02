use bevy::prelude::*;
use bevy::render::pipeline::PrimitiveTopology;

use noise::NoiseFn;
use noise::OpenSimplex;
use noise::Seedable;

pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub y: i32,
    pub index: [[[u8; 32]; 32]; 32],
}
impl Chunk {
    //creates a new empty chunk filled with air
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Chunk {x: x, y: y, z: z, index: [[[0u8; 32]; 32]; 32]}
    }
}

pub struct World {
    pub chunk_index: Vec<Chunk>,
    pub seed: u32,
}
impl World {
    pub fn new(seed: u32) -> Self {
        let chunk_index: Vec<Chunk> = Vec::new();

        World {seed: seed, chunk_index: chunk_index}
    }
}

pub fn spawn_world(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,  
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane{ size: 1.0 })),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(World::new(1457087));
}

pub fn generate_spawn(
    mut world: Query<&mut World, With<World>>
) {
    let radius = 3;
    for mut world in world.iter_mut() {
        for x in -radius..radius {
            for y in -radius..radius {
                for z in -radius..radius {
                    world.chunk_index.push(Chunk::new(x, y, z));

                    let current_chunk: usize = world.chunk_index.len() - 1;
                    world.chunk_index[current_chunk].index = generate_terrain(x, y, z, world.seed);
                }
            }
        } 
    }
    
}

fn generate_terrain(
    chunk_x: i32,
    chunk_y: i32,
    chunk_z: i32,
    seed: u32,
) -> [[[u8; 32]; 32]; 32] {
    let mut terrain: [[[u8; 32]; 32]; 32] = [[[0u8; 32]; 32]; 32];

    let open_simplex = OpenSimplex::new();
    open_simplex.set_seed(seed);
    
    for x in 0..32 {
        for z in 0..32 {
            //generates terrain with noise:
            let height: i32 = (open_simplex.get([
                ( (x as i32 + chunk_x * 32 as i32) as f32 / 15. ) as f64, 
                ( (z as i32 + chunk_z * 32 as i32) as f32 / 15. ) as f64,
            ]) * 15. + 48.0) as i32;

            let height_stalagmites: i32 = (open_simplex.get([
                ( (x as i32 + chunk_x * 5 as i32) as f32 / 5. ) as f64, 
                ( (z as i32 + chunk_z * 5 as i32) as f32 / 5. ) as f64,
            ]) * 42. + 15.) as i32;

            //writes height into terrain block index:
            for height_grass in height-3..height {
                if height_grass >= (chunk_y*32) as i32
                && height_grass <= (chunk_y*32 + 31) as i32 {
                    terrain[x][(height_grass - (chunk_y*32) as i32) as usize][z] = 1;
                }
            }
            

            //generates dirt
            for height_dirt in height-13..height-3 {
                if height_dirt >= (chunk_y*32) as i32
                && height_dirt <= (chunk_y*32 + 31) as i32 {
                    terrain[x][(height_dirt - (chunk_y*32) as i32) as usize][z] = 2;
                }
            }

            //generates stone
            for height_stone in -5..height-13 {
                if height_stone >= (chunk_y*32) as i32
                && height_stone <= (chunk_y*32 + 31) as i32 {
                    terrain[x][(height_stone - (chunk_y*32) as i32) as usize][z] = 3;
                }
            }

            //creates stalagmites
            for stalagmite in height as i32 - 64..height as i32 / 2 - 22 {
                if stalagmite >= (chunk_y*32) as i32
                && stalagmite <= (chunk_y*32 + 31) as i32 {
                    terrain[x][(stalagmite - (chunk_y*32) as i32) as usize][z] = 5;
                }
            }
            for stalagmite in -height_stalagmites..height as i32 - 64 {
                if stalagmite >= (chunk_y*32) as i32
                && stalagmite <= (chunk_y*32 + 31) as i32 {
                    terrain[x][(stalagmite - (chunk_y*32) as i32) as usize][z] = 4;
                }
            }

        }
    }
    terrain
}


use bevy::render::mesh::Indices;
use crate::Materials;


pub fn render_chunk(
    commands: &mut Commands,
    materials: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
    world: Query<&World, With<World>>,
) {
    for world in world.iter() {

        for chunk in 0..world.chunk_index.len() - 1 {

            
            let mesh = create_chunk_mesh(chunk, world);


            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(mesh),
                    material: materials.blocks.clone(),
                    transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                        Vec3::splat(1.0),
                        Quat::from_rotation_x(0.0),
                        Vec3::new((world.chunk_index[chunk].x * 32) as f32, (world.chunk_index[chunk].y * 32) as f32, (world.chunk_index[chunk].z * 32) as f32),
                    )),
                    ..Default::default()
                });
        }       
    }
}

fn create_chunk_mesh(
    chunk: usize,
    world: &World,
) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let v_length = 8*32*32*32;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(v_length);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(v_length);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(v_length);

    let mut indices: Vec<u32> = Vec::with_capacity(v_length);

    for x1 in 0..32 {
        for y1 in 0..32 {
            for z1 in 0..32 {

                let x: f32 = x1 as f32;
                let y: f32 = y1 as f32;
                let z: f32= z1 as f32;

                let block: u8 = world.chunk_index[chunk].index[x1][y1][z1];

                let world_x = world.chunk_index[chunk].x as f32 * 32.0 + x;
                let world_y = world.chunk_index[chunk].y as f32 * 32.0 + y;
                let world_z = world.chunk_index[chunk].z as f32 * 32.0 + z;
                
                if block != 0 { 

                    // left side
                    positions.push([ x, y, z ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ block as f32 / 256.0, 0.0 ]);

                    positions.push([ x + 1.0, y, z ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ (block + 1) as f32 / 256.0, 0.0 ]);

                    positions.push([ x, y + 1.0, z ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ block as f32 / 256.0, 1.0 ]);

                    positions.push([ x + 1.0, y + 1.0, z ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ (block + 1) as f32 / 256.0, 1.0 ]);


                    // rear side
                    positions.push([ x + 1.0, y, z ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ block as f32 / 256.0, 0.0 ]);

                    positions.push([ x + 1.0, y, z + 1.0 ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ (block + 1) as f32 / 256.0, 0.0 ]);

                    positions.push([ x + 1.0, y + 1.0, z ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ block as f32 / 256.0, 1.0 ]);

                    positions.push([ x + 1.0, y + 1.0, z + 1.0 ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ (block + 1) as f32 / 256.0, 1.0 ]);


                    // right side
                    positions.push([ x, y, z + 1.0 ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ block as f32 / 256.0, 0.0 ]);

                    positions.push([ x + 1.0, y, z + 1.0 ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ (block + 1) as f32 / 256.0, 0.0 ]);

                    positions.push([ x, y + 1.0, z + 1.0 ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ block as f32 / 256.0, 1.0 ]);

                    positions.push([ x + 1.0, y + 1.0, z + 1.0 ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ (block + 1) as f32 / 256.0, 1.0 ]);


                    // front side
                    positions.push([ x, y, z ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ block as f32 / 256.0, 0.0 ]);

                    positions.push([ x, y, z + 1.0 ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ (block + 1) as f32 / 256.0, 0.0 ]);

                    positions.push([ x, y + 1.0, z ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ block as f32 / 256.0, 1.0 ]);

                    positions.push([ x, y + 1.0, z + 1.0 ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ (block + 1) as f32 / 256.0, 1.0 ]);


                    // bottom side
                    positions.push([ x, y, z ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ block as f32 / 256.0, 0.0 ]);

                    positions.push([ x, y, z + 1.0 ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ (block + 1) as f32 / 256.0, 0.0 ]);

                    positions.push([ x + 1.0, y, z ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ block as f32 / 256.0, 1.0 ]);

                    positions.push([ x + 1.0, y, z + 1.0 ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ (block + 1) as f32 / 256.0, 1.0 ]);


                    // upper side
                    positions.push([ x, y + 1.0, z ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ block as f32 / 256.0, 0.0 ]);

                    positions.push([ x, y + 1.0, z + 1.0 ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ (block + 1) as f32 / 256.0, 0.0 ]);

                    positions.push([ x + 1.0, y + 1.0, z ]);
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ block as f32 / 256.0, 1.0 ]);

                    positions.push([ x + 1.0, y + 1.0, z + 1.0] );
                    normals.push([ world_x, world_y, world_z ]);
                    uvs.push([ (block + 1) as f32 / 256.0, 1.0 ]);



                    //creates indices
                
                    //below plane
                    let below = create_indices("below", positions.len() as u32);
                    if y as usize >= 1 {
                        if world.chunk_index[chunk].index[x1][y1-1][z1] == 0 {
                            for i in 0..6 { indices.push(below[i]) }
                        }
                    } else {
                        for i in 0..6 { indices.push(below[i]) }
                    }

                    //above plane
                    let above = create_indices("above", positions.len() as u32);
                    if y as usize <= 30 {
                        if world.chunk_index[chunk].index[x1][y1+1][z1] == 0 {
                            for i in 0..6 { indices.push(above[i]) }
                        }
                    } else {
                        for i in 0..6 { indices.push(above[i]) }
                    }

                    //left plane
                    let left = create_indices("left", positions.len() as u32);
                    if z as usize >= 1 {
                        if world.chunk_index[chunk].index[x1][y1][z1-1] == 0 {
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
                    if z as usize <= 30 {
                        if world.chunk_index[chunk].index[x1][y1][z1+1] == 0 {
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
                    if x as usize >= 1 {
                       if world.chunk_index[chunk].index[x1-1][y1][z1] == 0 {
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
                    if x as usize <= 30 {
                        if world.chunk_index[chunk].index[x1+1][y1][z1] == 0 {
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
    
    mesh
}


fn create_indices(
    side: &str,
    position_len: u32,
) -> [u32; 6] {
    let mut indices: [u32; 6] = [0, 0, 0, 0, 0, 0];

    if side == "below" {
        indices[0] = (position_len - 8 + 0) as u32;
        indices[1] = (position_len - 8 + 2) as u32;
        indices[2] = (position_len - 8 + 1) as u32;

        indices[3] = (position_len - 8 + 3) as u32;
        indices[4] = (position_len - 8 + 1) as u32;
        indices[5] = (position_len - 8 + 2) as u32;
    }

    if side == "above" {
        indices[0] = (position_len - 4 + 2) as u32;
        indices[1] = (position_len - 4 + 0) as u32;
        indices[2] = (position_len - 4 + 3) as u32;

        indices[3] = (position_len - 4 + 0) as u32;
        indices[4] = (position_len - 4 + 1) as u32;
        indices[5] = (position_len - 4 + 3) as u32;
    }

    if side == "left" {
        indices[0] = (position_len - 24 + 1) as u32;
        indices[1] = (position_len - 24 + 0) as u32;
        indices[2] = (position_len - 24 + 3) as u32;

        indices[3] = (position_len - 24 + 0) as u32;
        indices[4] = (position_len - 24 + 2) as u32;
        indices[5] = (position_len - 24 + 3) as u32;
    }

    if side == "right" {
        indices[0] = (position_len - 16 + 0) as u32;
        indices[1] = (position_len - 16 + 1) as u32;
        indices[2] = (position_len - 16 + 3) as u32;

        indices[3] = (position_len - 16 + 2) as u32;
        indices[4] = (position_len - 16 + 0) as u32;
        indices[5] = (position_len - 16 + 3) as u32;        
    }

    if side == "front" {
        indices[0] = (position_len - 12 + 0) as u32;
        indices[1] = (position_len - 12 + 1) as u32;
        indices[2] = (position_len - 12 + 2) as u32;

        indices[3] = (position_len - 12 + 2) as u32;
        indices[4] = (position_len - 12 + 1) as u32;
        indices[5] = (position_len - 12 + 3) as u32;        
    }

    if side == "back" {
        indices[0] = (position_len - 20 + 1) as u32;
        indices[1] = (position_len - 20 + 0) as u32;
        indices[2] = (position_len - 20 + 2) as u32;

        indices[3] = (position_len - 20 + 1) as u32;
        indices[4] = (position_len - 20 + 2) as u32;
        indices[5] = (position_len - 20 + 3) as u32;
    }
    indices
}