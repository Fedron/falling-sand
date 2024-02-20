use wgpu::util::DeviceExt;

use crate::render::{drawable::Drawable, pipeline::Vertex};

pub struct Quad {
    pub position: cgmath::Point2<f32>,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl Quad {
    const VERTICES: [Vertex; 4] = [
        Vertex {
            // Top-left
            position: [0.0, 1.0],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            // Top-right
            position: [1.0, 1.0],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            // Bottom-right
            position: [1.0, 0.0],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            // Bottom-left
            position: [0.0, 0.0],
            tex_coords: [0.0, 1.0],
        },
    ];

    const INDICES: [u16; 6] = [0, 1, 3, 1, 2, 3];

    pub fn new(device: &wgpu::Device, quad_size: (usize, usize)) -> Self {
        let mut vertices = Self::VERTICES.to_vec();
        for vertex in &mut vertices {
            vertex.position[0] *= quad_size.0 as f32;
            vertex.position[1] *= quad_size.1 as f32;
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Quad Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Quad Index Buffer"),
            contents: bytemuck::cast_slice(&Self::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            position: cgmath::Point2::new(0.0, 0.0),
            vertex_buffer,
            index_buffer,
        }
    }
}

impl Drawable for Quad {
    fn get_vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    fn get_num_vertices(&self) -> u32 {
        Self::VERTICES.len() as u32
    }

    fn get_index_buffer(&self) -> Option<&wgpu::Buffer> {
        Some(&self.index_buffer)
    }

    fn get_num_indices(&self) -> Option<u32> {
        Some(Self::INDICES.len() as u32)
    }
}
