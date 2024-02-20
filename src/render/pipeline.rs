use std::{borrow::Cow, cell::RefCell, rc::Rc};

use crate::{
    camera::{Camera, CameraUniform},
    texture::Texture,
    ModelUniform,
};

use super::renderer::Renderer;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub struct RenderPipeline2D {
    renderer: Rc<RefCell<Renderer>>,
    render_pipeline: wgpu::RenderPipeline,

    pub texture: Texture,
    texture_bind_group: wgpu::BindGroup,

    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    model_uniform: ModelUniform,
    model_buffer: wgpu::Buffer,
    model_bind_group: wgpu::BindGroup,
}

impl RenderPipeline2D {
    // TODO: Maybe use builder pattern to supply the texture, camera, and model
    pub fn new(renderer: Rc<RefCell<Renderer>>) -> Self {
        let shader = renderer
            .borrow()
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Render Pipeline Shader"),
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
            });

        let (texture, texture_bind_group_layout, texture_bind_group) =
            Self::create_texture(renderer.clone());

        let (camera_uniform, camera_buffer, camera_bind_group_layout, camera_bind_group) =
            Self::create_camera_buffer(renderer.clone());

        let (model_uniform, model_buffer, model_bind_group_layout, model_bind_group) =
            Self::create_model_buffer(renderer.clone());

        let pipeline_layout =
            renderer
                .borrow()
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline 2D Layout"),
                    bind_group_layouts: &[
                        &texture_bind_group_layout,
                        &camera_bind_group_layout,
                        &model_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

        let render_pipeline =
            renderer
                .borrow()
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline 2D"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main",
                        buffers: &[Vertex::desc()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(renderer.borrow().swapchain_format.into())],
                    }),
                    primitive: wgpu::PrimitiveState {
                        front_face: wgpu::FrontFace::Cw,
                        cull_mode: Some(wgpu::Face::Back),
                        ..Default::default()
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    multiview: None,
                });

        Self {
            renderer,
            render_pipeline,

            texture,
            texture_bind_group,

            camera_uniform,
            camera_buffer,
            camera_bind_group,

            model_uniform,
            model_buffer,
            model_bind_group,
        }
    }

    fn create_texture(
        renderer: Rc<RefCell<Renderer>>,
    ) -> (Texture, wgpu::BindGroupLayout, wgpu::BindGroup) {
        let texture = Texture::new(&renderer.borrow().device, 64, 64);

        let texture_bind_group_layout =
            renderer
                .borrow()
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Texture Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: texture.texture_binding(),
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: texture.sampler_binding(),
                            count: None,
                        },
                    ],
                });

        let texture_bind_group =
            renderer
                .borrow()
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Texture Bind Group"),
                    layout: &texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&texture.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&texture.sampler),
                        },
                    ],
                });

        (texture, texture_bind_group_layout, texture_bind_group)
    }

    fn create_camera_buffer(
        renderer: Rc<RefCell<Renderer>>,
    ) -> (
        CameraUniform,
        wgpu::Buffer,
        wgpu::BindGroupLayout,
        wgpu::BindGroup,
    ) {
        let camera_uniform = CameraUniform::new();

        let camera_buffer = renderer
            .borrow()
            .device
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Camera Uniform Buffer"),
                size: std::mem::size_of::<CameraUniform>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        let camera_bind_group_layout =
            renderer
                .borrow()
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Camera Bind Group Layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        let camera_bind_group =
            renderer
                .borrow()
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Camera Bind Group"),
                    layout: &camera_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_buffer.as_entire_binding(),
                    }],
                });

        (
            camera_uniform,
            camera_buffer,
            camera_bind_group_layout,
            camera_bind_group,
        )
    }

    fn create_model_buffer(
        renderer: Rc<RefCell<Renderer>>,
    ) -> (
        ModelUniform,
        wgpu::Buffer,
        wgpu::BindGroupLayout,
        wgpu::BindGroup,
    ) {
        let model_uniform = ModelUniform::new();

        let model_buffer = renderer
            .borrow()
            .device
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Model Uniform Buffer"),
                size: std::mem::size_of::<ModelUniform>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        let model_bind_group_layout =
            renderer
                .borrow()
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Model Bind Group Layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        let model_bind_group =
            renderer
                .borrow()
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Model Bind Group"),
                    layout: &model_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: model_buffer.as_entire_binding(),
                    }],
                });

        (
            model_uniform,
            model_buffer,
            model_bind_group_layout,
            model_bind_group,
        )
    }
}

impl RenderPipeline2D {
    pub fn update_camera(&mut self, camera: &Camera) {
        self.camera_uniform.update_view_projection(camera);
        self.renderer.borrow().queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    pub fn update_model(&mut self, position: cgmath::Point3<f32>) {
        self.model_uniform.update_model(position);
        self.renderer.borrow().queue.write_buffer(
            &self.model_buffer,
            0,
            bytemuck::cast_slice(&[self.model_uniform]),
        );
    }

    pub fn prepare<'a: 'b, 'b>(&'a self, render_pass: &mut wgpu::RenderPass<'b>) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
        render_pass.set_bind_group(2, &self.model_bind_group, &[]);
    }
}
