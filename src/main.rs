use camera::{Camera, CameraUniform};
use chunk::Chunk;
use render::{
    context::RenderContext,
    renderer::{Renderer, Vertex},
};
use std::{cell::RefCell, rc::Rc, sync::Arc};
use texture::Texture;
use wgpu::util::DeviceExt;
use window::{Application, WindowManager};
use winit_input_helper::WinitInputHelper;

mod camera;
mod cell;
mod chunk;
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
    render_context: Rc<RefCell<RenderContext>>,
    renderer: Renderer,

    last_update: std::time::Instant,
    update_counter: usize,

    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,

    chunk: Chunk,
    texture: Texture,
    texture_pixels: Vec<u8>,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl FallingSandApplication {
    pub fn new(window: Arc<winit::window::Window>) -> Self {
        let render_context = Rc::new(RefCell::new(pollster::block_on(RenderContext::new(
            window.clone(),
        ))));

        let context = render_context.borrow();

        let chunk = Chunk::new();
        let texture = Texture::new(&context.device, 64, 64);
        let mut texture_pixels: Vec<u8> = Vec::with_capacity(64 * 64 * 4);
        for _ in 0..64 * 64 {
            texture_pixels.extend_from_slice(&[0, 0, 0, 0]);
        }

        let size = window.inner_size();
        let mut camera = Camera::new(size.width as f32, size.height as f32);
        camera.position = (-(size.width as f32 / 2.0), -(size.height as f32 / 2.0), 0.0).into();

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_projection(&camera);

        let camera_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let renderer = Renderer::new(render_context.clone(), &texture, &camera_buffer);

        let vertex_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });
        let num_indices = INDICES.len() as u32;

        drop(context);

        Self {
            render_context,
            renderer,

            last_update: std::time::Instant::now(),
            update_counter: 0,

            camera,
            camera_uniform,
            camera_buffer,

            chunk,
            texture,
            texture_pixels,

            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }
}

impl Application for FallingSandApplication {
    fn update(&mut self) {
        let now = std::time::Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f32();
        if delta_time < 0.2 {
            return;
        }

        self.chunk.update(self.update_counter);
        self.last_update = now;
        self.update_counter += 1;
    }

    fn draw(&mut self) {
        self.chunk.draw(&mut self.texture_pixels);
        self.texture
            .upload_pixels(&self.render_context.borrow().queue, &self.texture_pixels);

        self.renderer
            .render(&self.vertex_buffer, &self.index_buffer, self.num_indices);
    }

    fn handle_input(&mut self, input: &WinitInputHelper) {
        if let Some(new_size) = input.window_resized() {
            self.render_context.borrow_mut().resize(new_size);

            self.camera
                .update_size(new_size.width as f32, new_size.height as f32);
            self.camera_uniform.update_view_projection(&self.camera);

            let context = self.render_context.borrow();
            context.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytemuck::cast_slice(&[self.camera_uniform]),
            )
        }
    }
}

fn main() {
    env_logger::init();

    let window_manager = WindowManager::new("Falling Sand", (800, 600));
    let app = Box::new(FallingSandApplication::new(window_manager.window.clone()));

    window_manager.run(app);
}
