use camera::{Camera, CameraUniform};
use render::{
    context::RenderContext,
    renderer::{Renderer, Vertex},
};
use std::{rc::Rc, sync::Arc};
use texture::Texture;
use wgpu::util::DeviceExt;
use window::{Application, WindowManager};
use winit_input_helper::WinitInputHelper;

mod camera;
mod render;
mod texture;
mod window;

const VERTICES: &[Vertex] = &[
    Vertex {
        // Top-left
        position: [-100.0, 100.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        // Top-right
        position: [100.0, 100.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        // Bottom-right
        position: [100.0, -100.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        // Bottom-left
        position: [-100.0, -100.0],
        tex_coords: [0.0, 1.0],
    },
];

const INDICES: &[u16] = &[0, 1, 3, 1, 2, 3];

struct FallingSandApplication {
    _render_context: Rc<RenderContext>,
    renderer: Renderer,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl FallingSandApplication {
    pub fn new(window: Arc<winit::window::Window>) -> Self {
        let render_context = Rc::new(pollster::block_on(RenderContext::new(window.clone())));

        let texture = Texture::new(&render_context.device, 64, 64);
        let mut pixels: Vec<u8> = Vec::with_capacity(64 * 64 * 4);
        for x in 0..64 {
            for y in 0..64 {
                let r = (x as f32 / 64.0 * 255.0) as u8;
                let g = (y as f32 / 64.0 * 255.0) as u8;
                let b = 0;
                let a = 255;
                pixels.push(r);
                pixels.push(g);
                pixels.push(b);
                pixels.push(a);
            }
        }
        texture.upload_pixels(&render_context.queue, &pixels);

        let size = window.inner_size();
        let mut camera = Camera::new(size.width as f32, size.height as f32);
        camera.position = (-(size.width as f32 / 2.0), -(size.height as f32 / 2.0), 0.0).into();

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_projection(&camera);

        let camera_buffer =
            render_context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&[camera_uniform]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let renderer = Renderer::new(render_context.clone(), &texture, &camera_buffer);

        let vertex_buffer =
            render_context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(VERTICES),
                    usage: wgpu::BufferUsages::VERTEX,
                });
        let index_buffer =
            render_context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(INDICES),
                    usage: wgpu::BufferUsages::INDEX,
                });
        let num_indices = INDICES.len() as u32;

        Self {
            _render_context: render_context,
            renderer,

            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }
}

impl Application for FallingSandApplication {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.renderer
            .render(&self.vertex_buffer, &self.index_buffer, self.num_indices);
    }

    fn handle_input(&mut self, _: &WinitInputHelper) {}
}

fn main() {
    env_logger::init();

    let window_manager = WindowManager::new("Falling Sand", (800, 600));
    let app = Box::new(FallingSandApplication::new(window_manager.window.clone()));

    window_manager.run(app);
}
