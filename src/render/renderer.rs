use std::sync::Arc;

pub struct Frame {
    pub texture: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
    pub encoder: wgpu::CommandEncoder,
}

pub struct Renderer {
    _window: Arc<winit::window::Window>,
    _instance: wgpu::Instance,
    _adapter: wgpu::Adapter,
    pub surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub swapchain_format: wgpu::TextureFormat,
}

impl Renderer {
    pub async fn new(window: Arc<winit::window::Window>) -> Self {
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let size = window.inner_size();
        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &surface_config);

        Self {
            _window: window,
            _instance: instance,
            _adapter: adapter,
            surface,
            surface_config,
            device,
            queue,
            swapchain_format,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.surface_config.width = new_size.width.max(1);
        self.surface_config.height = new_size.height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn begin_render(&mut self) -> Option<Frame> {
        if let Ok(frame) = self.surface.get_current_texture() {
            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            return Some(Frame {
                texture: frame,
                view,
                encoder,
            });
        }

        None
    }

    pub fn finish_render(&self, frame: Frame) {
        self.queue.submit(Some(frame.encoder.finish()));
        frame.texture.present();
    }
}
