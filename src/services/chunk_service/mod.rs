//
// Handles chunk loading, chunk unloading and chunk animations
//

use crate::services::settings_service::SettingsService;
use crate::services::ServicesContext;
use wgpu::{BindGroupLayout, Device};
use crate::world::generator::World;
use crate::services::chunk_service::chunk::{Chunk, ChunkData};
use crate::block::Block;
use cgmath::{Vector3};
use std::collections::HashMap;

pub mod mesh;
pub mod chunk;

pub struct ChunkService {
    pub(crate) bind_group_layout: BindGroupLayout,
    pub(crate) chunks: HashMap<Vector3<i32>, Chunk>,
    pub(crate) vertices_count: u64,
    pub(crate) chunk_keys: Vec<Vector3<i32>>
}

impl ChunkService {

    pub fn new(settings: &SettingsService, context: &mut ServicesContext) -> ChunkService {

        let bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: true
                    },
                }
            ]
        };

        // Create the chunk bind group layout
        let bind_group_layout = context.device.create_bind_group_layout(&bind_group_layout_descriptor);

        let mut service = ChunkService {
            bind_group_layout,
            chunks: HashMap::new(),
            vertices_count: 0,
            chunk_keys: Vec::new()
        };

        //TODO: Remove this once we have networking
        for x in -(settings.render_distance as i32)..(settings.render_distance as i32 * 8) {
            for z in -(settings.render_distance as i32)..(settings.render_distance as i32) {
                for y in 0..16 {
                    let data = ChunkService::generate_chunk(x, y, z, context.blocks);
                    service.load_chunk(context.device, data, Vector3 { x, y, z }, &settings);
                }
            }
        }

        for i in 0..service.chunks.len() {
            let chunk_key = service.chunk_keys
                .get(i)
                .unwrap();

            let mesh_data = {
                let chunk = service.chunks
                    .get(chunk_key)
                    .unwrap();

                chunk.generate_mesh(&service)
            };

            // Add new vertices to count
            service.vertices_count += mesh_data.vertices.len() as u64;

            let chunk = service.chunks
                .get_mut(chunk_key)
                .unwrap();

            chunk.update_mesh(mesh_data);
            chunk.create_buffers(&context.device, &service.bind_group_layout);
        }

        service
    }

    //TODO: Remove this once we have networking setup
    fn generate_chunk(x: i32, y: i32, z: i32, blocks: &Vec<Block>) -> ChunkData {
        return World::generate_chunk(Vector3 { x, y, z }, blocks);
    }

    pub fn load_chunk(&mut self, device: &Device, data: ChunkData, chunk_coords: Vector3<i32>, settings: &SettingsService) {
        let mut chunk = Chunk::new(data, chunk_coords);

        // chunk.generate_mesh(&self);
        // self.vertices += chunk.vertices.as_ref().unwrap().len() as u64;
        // chunk.create_buffers(device, &self.bind_group_layout);

        self.chunk_keys.push(chunk_coords.clone());
        self.chunks.insert(chunk_coords, chunk);
    }
}