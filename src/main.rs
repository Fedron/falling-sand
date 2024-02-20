use bbox::BoundingBox;
use camera::{Camera, CameraUniform};
use chunk::Chunk;
use render::{drawable::Drawable, pipeline::RenderPipeline2D, renderer::Renderer};
use std::sync::Arc;
use texture::{Texture, TexturedQuad};
use wgpu::util::DeviceExt;
use window::{Application, WindowManager};
use winit_input_helper::WinitInputHelper;

mod bbox;
mod camera;
mod cell;
mod chunk;
mod render;
mod texture;
mod window;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelUniform {
    model: [[f32; 4]; 4],
}

impl ModelUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            model: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_model(&mut self, position: cgmath::Point3<f32>) {
        use cgmath::EuclideanSpace;
        self.model = cgmath::Matrix4::from_translation(position.to_vec()).into()
    }
}

struct FallingSandApplication {
    renderer: Renderer,
    render_pipeline: RenderPipeline2D,

    last_update: std::time::Instant,
    update_counter: usize,

    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,

    chunk: Chunk,
    chunk_bbox: BoundingBox,

    texture: Texture,
    texture_pixels: Vec<u8>,
    textured_quad: TexturedQuad,
}

impl FallingSandApplication {
    pub fn new(window: Arc<winit::window::Window>) -> Self {
        let renderer = pollster::block_on(Renderer::new(window.clone()));

        let chunk = Chunk::new();
        let chunk_bbox = BoundingBox {
            min: (128.0, 128.0).into(),
            max: (256.0, 256.0).into(),
        };

        let mut chunk_uniform = ModelUniform::new();
        chunk_uniform.update_model((128.0, 128.0, 0.0).into());

        let chunk_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Chunk Uniform Buffer"),
                contents: bytemuck::cast_slice(&[chunk_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let texture = Texture::new(&renderer.device, 64, 64);
        let textured_quad = TexturedQuad::new(&renderer.device, (128, 128));
        let mut texture_pixels: Vec<u8> = Vec::with_capacity(64 * 64 * 4);
        for _ in 0..64 * 64 {
            texture_pixels.extend_from_slice(&[0, 0, 0, 0]);
        }

        let size = window.inner_size();
        let camera = Camera::new(size.width as f32, size.height as f32);

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_projection(&camera);

        let camera_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Uniform Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let render_pipeline = RenderPipeline2D::new(
            &renderer.device,
            renderer.swapchain_format.into(),
            &texture,
            &camera_buffer,
            &chunk_buffer,
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
            chunk_bbox,

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
            {
                let mut render_pass = self.renderer.create_default_render_pass(&mut frame);
                self.render_pipeline.prepare(&mut render_pass);
                self.textured_quad.draw(&mut render_pass);
            }

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

        if input.mouse_held(0) {
            if let Some((x, y)) = input.cursor() {
                let world_pos = self
                    .camera
                    .window_pos_to_world_pos((x as f32, y as f32).into());

                if self.chunk_bbox.contains(world_pos.into()) {
                    println!("In chunk: {:?}", world_pos);
                }
            }
        }
    }
}

fn main() {
    env_logger::init();

    let window_manager = WindowManager::new("Falling Sand", (800, 600));
    let app = Box::new(FallingSandApplication::new(window_manager.window.clone()));

    window_manager.run(app);
}
