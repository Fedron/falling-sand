use wgpu::util::DeviceExt;

use crate::render::{drawable::Drawable, pipeline::Vertex};

pub struct Texture {
    width: usize,
    height: usize,

    texture: wgpu::Texture,
    extent: wgpu::Extent3d,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn new(device: &wgpu::Device, width: usize, height: usize) -> Self {
        let extent = wgpu::Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Texture {
            width,
            height,

            texture,
            extent,
            view,
            sampler,
        }
    }

    pub fn texture_binding(&self) -> wgpu::BindingType {
        wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        }
    }

    pub fn sampler_binding(&self) -> wgpu::BindingType {
        wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
    }

    pub fn upload_pixels(&self, queue: &wgpu::Queue, pixels: &[u8]) {
        if pixels.len() != self.width * self.height * 4 {
            panic!("Cannot upload pixel data to texture of size {:?}x{:?} when provided pixel data does not match", self.width, self.height);
        }

        queue.write_texture(
            self.texture.as_image_copy(),
            &pixels,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(self.width as u32 * 4),
                rows_per_image: Some(self.height as u32),
            },
            self.extent,
        );
    }
}

pub struct TexturedQuad {
    pub texture: Texture,
    pub position: cgmath::Point2<f32>,

    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl TexturedQuad {
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
        let texture = Texture::new(&device, quad_size.0, quad_size.1);

        let mut vertices = Self::VERTICES.to_vec();
        for vertex in &mut vertices {
            vertex.position[0] *= quad_size.0 as f32;
            vertex.position[1] *= quad_size.1 as f32;
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("TexturedQuad Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("TexturedQuad Index Buffer"),
            contents: bytemuck::cast_slice(&Self::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            texture,
            position: cgmath::Point2::new(0.0, 0.0),

            vertex_buffer,
            index_buffer,
        }
    }
}

impl Drawable for TexturedQuad {
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
