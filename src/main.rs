use camera::{Camera, CameraUniform};
use chunk::Chunk;
use render::{pipeline::RenderPipeline2D, renderer::Renderer};
use std::sync::Arc;
use texture::{Texture, TexturedQuad};
use wgpu::util::DeviceExt;
use window::{Application, WindowManager};
use winit_input_helper::WinitInputHelper;

mod camera;
mod cell;
mod chunk;
mod render;
mod texture;
mod window;

struct FallingSandApplication {
    renderer: Renderer,
    render_pipeline: RenderPipeline2D,

    last_update: std::time::Instant,
    update_counter: usize,

    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,

    chunk: Chunk,
    texture: Texture,
    texture_pixels: Vec<u8>,
    textured_quad: TexturedQuad,
}

impl FallingSandApplication {
    pub fn new(window: Arc<winit::window::Window>) -> Self {
        let renderer = pollster::block_on(Renderer::new(window.clone()));

        let chunk = Chunk::new();
        let texture = Texture::new(&renderer.device, 64, 64);
        let mut texture_pixels: Vec<u8> = Vec::with_capacity(64 * 64 * 4);
        for _ in 0..64 * 64 {
            texture_pixels.extend_from_slice(&[0, 0, 0, 0]);
        }

        let textured_quad = TexturedQuad::new(&renderer.device, (128, 128));

        let size = window.inner_size();
        let mut camera = Camera::new(size.width as f32, size.height as f32);
        camera.position = (-(size.width as f32 / 2.0), -(size.height as f32 / 2.0), 0.0).into();

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_projection(&camera);

        let camera_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let render_pipeline = RenderPipeline2D::new(
            &renderer.device,
            renderer.swapchain_format.into(),
            &texture,
            &camera_buffer,
        );

        Self {
            renderer,
            render_pipeline,

            last_update: std::time::Instant::now(),
            update_counter: 0,

            camera,
            camera_uniform,
            camera_buffer,

            chunk,
            texture,
            texture_pixels,
            textured_quad,
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
            .upload_pixels(&self.renderer.queue, &self.texture_pixels);

        if let Some(mut frame) = self.renderer.begin_render() {
            self.render_pipeline.render(&mut frame, &self.textured_quad);
            self.renderer.finish_render(frame);
        }
    }

    fn handle_input(&mut self, input: &WinitInputHelper) {
        if let Some(new_size) = input.window_resized() {
            self.renderer.resize(new_size);

            self.camera
                .update_size(new_size.width as f32, new_size.height as f32);
            self.camera_uniform.update_view_projection(&self.camera);

            self.renderer.queue.write_buffer(
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
