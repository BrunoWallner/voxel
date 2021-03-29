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
    let radius = 5;
    for mut world in world.iter_mut() {
        for x in -radius..radius {
            for y in 0..radius*2 {
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
            let height: u32 = (open_simplex.get([
                ( (x as i32 + chunk_x * 32 as i32) as f32 / 15. ) as f64, 
                ( (z as i32 + chunk_z * 32 as i32) as f32 / 15. ) as f64,
            ]) * 15. + 20.0) as u32;

            //writes height into terrain block index:
            if height >= (chunk_y*32) as u32
            && height <= (chunk_y*32 + 31) as u32 {
                terrain[x][(height - (chunk_y*32) as u32) as usize][z] = 1;
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

    let mut indices: Vec<u32> = Vec::with_capacity(v_length / 8);

    for x in 0..32 {
        for y in 0..32 {
            for z in 0..32 {
                if world.chunk_index[chunk].index[x][y][z] != 0 {

                    //creates vertices
                    let uv1: f32 = 0.0;
                    let uv2: f32 = 0.1;
                    for i in 0..2 {
                        positions.push([x as f32, (y + i) as f32, z as f32]);
                        normals.push([ (world.chunk_index[chunk].x*32 + x as i32) as f32, (world.chunk_index[chunk].y*32 + y as i32) as f32, (world.chunk_index[chunk].z*32 + z as i32) as f32  ]);
                        uvs.push([uv1, uv2]);

                        positions.push([(x + 1) as f32, (y + i) as f32, z as f32]);
                        normals.push([ (world.chunk_index[chunk].x*32 + x as i32) as f32, (world.chunk_index[chunk].y*32 + y as i32) as f32, (world.chunk_index[chunk].z*32 + z as i32) as f32  ]);
                        uvs.push([uv1, uv2]);

                        positions.push([(x + 1) as f32, (y + i) as f32, (z + 1) as f32]);
                        normals.push([ (world.chunk_index[chunk].x*32 + x as i32) as f32, (world.chunk_index[chunk].y*32 + y as i32) as f32, (world.chunk_index[chunk].z*32 + z as i32) as f32  ]);
                        uvs.push([uv1, uv2]);

                        positions.push([x as f32, (y + i) as f32, (z + 1) as f32]);
                        normals.push([ (world.chunk_index[chunk].x*32 + x as i32) as f32, (world.chunk_index[chunk].y*32 + y as i32) as f32, (world.chunk_index[chunk].z*32 + z as i32) as f32  ]);
                        uvs.push([uv1, uv2]);
                    }

                    //creates indices
                
                    //below plane
                    let below = create_indices("below", positions.len() as u32);
                    if y >= 1 {
                        if world.chunk_index[chunk].index[x][y-1][z] == 0 {
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
                    if y <= 30 {
                        if world.chunk_index[chunk].index[x][y+1][z] == 0 {
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
                        if world.chunk_index[chunk].index[x][y][z-1] == 0 {
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
                    if z <= 30 {
                        if world.chunk_index[chunk].index[x][y][z+1] == 0 {
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
                       if world.chunk_index[chunk].index[x-1][y][z] == 0 {
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
                    if x <= 30 {
                        if world.chunk_index[chunk].index[x+1][y][z] == 0 {
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













































/* Chunk spawning and despawning

use crate::Materials;

use std::time::Duration;

pub struct ChunkTickTimer(Timer);
impl Default for ChunkTickTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(100), true))
    }
}

pub fn chunk_loader(
    camera: Query<&Transform, With<Camera>>,
    mut chunk: Query<&mut Chunk, With<Chunk>>,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    mut chunk_tick_timer: Local<ChunkTickTimer>,
) {
    if chunk_tick_timer.0.tick(time.delta_seconds()).finished() {
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
                        transform: Transform::from_translation(Vec3::new((camera_chunk[0]*16 + 8) as f32, 0.0, (camera_chunk[1]*16 + 8) as f32)),
                        ..Default::default()
                    })
                    .with(Chunk::new(camera_chunk[0], camera_chunk[1]));
                }
            }
        }
    }
}


pub fn chunk_unloader(
    camera: Query<&Transform, With<Camera>>,
    mut rendered_chunk: Query<(&RenderedChunk, Entity), With<RenderedChunk>>,
    mut chunk: Query<(&Chunk, Entity), With<Chunk>>,
    commands: &mut Commands,
    time: Res<Time>,
    mut chunk_tick_timer: Local<ChunkTickTimer>,
) {
    if chunk_tick_timer.0.tick(time.delta_seconds()).finished() {
        let render_distance: i32 = 15;
        
        let mut camera_chunk: [i32; 2] = [0, 0];
        for camera in camera.iter() {
            camera_chunk = [(camera.translation.x / 16.0) as i32, (camera.translation.z / 16.0) as i32]
        }

        for (chunk, entity) in rendered_chunk.iter_mut() {
            if chunk.x > camera_chunk[0] + render_distance
            || chunk.x < camera_chunk[0] - render_distance
            || chunk.z > camera_chunk[1] + render_distance
            || chunk.z < camera_chunk[1] - render_distance {
                commands.despawn(entity);
            }
        }
        for (chunk, entity) in chunk.iter_mut() {
            if chunk.x > camera_chunk[0] + render_distance
            || chunk.x < camera_chunk[0] - render_distance
            || chunk.z > camera_chunk[1] + render_distance
            || chunk.z < camera_chunk[1] - render_distance {
                commands.despawn(entity);
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
            let v_length = 8*16*16*256;

            let mut positions: Vec<[f32; 3]> = Vec::with_capacity(v_length);
            let mut normals: Vec<[f32; 3]> = Vec::with_capacity(v_length);
            let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(v_length);

            let mut indices: Vec<u32> = Vec::with_capacity(v_length / 8);
        
            for x in 0..16 {
                for y in 0..256 {
                    for z in 0..16 {
                        if chunk.index[x][y][z] != 0 {

                            //creates vertices
                            let uv1: f32 = 0.1;
                            let uv2: f32 = 0.2;
                            for i in 0..2 {
                                positions.push([x as f32, (y + i) as f32, z as f32]);
                                normals.push([ (chunk.x*16 + x as i32) as f32, y as f32, (chunk.z*16 + z as i32) as f32  ]);
                                uvs.push([uv1, uv2]);

                                positions.push([(x + 1) as f32, (y + i) as f32, z as f32]);
                                normals.push([ (chunk.x*16 + x as i32) as f32, y as f32, (chunk.z*16 + z as i32) as f32  ]);
                                uvs.push([uv1, uv2]);

                                positions.push([(x + 1) as f32, (y + i) as f32, (z + 1) as f32]);
                                normals.push([ (chunk.x*16 + x as i32) as f32, y as f32, (chunk.z*16 + z as i32) as f32  ]);
                                uvs.push([uv1, uv2]);

                                positions.push([x as f32, (y + i) as f32, (z + 1) as f32]);
                                normals.push([ (chunk.x*16 + x as i32) as f32, y as f32, (chunk.z*16 + z as i32) as f32  ]);
                                uvs.push([uv1, uv2]);
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
                    material: materials.blocks.clone(),
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
*/