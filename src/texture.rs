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
            mag_filter: wgpu::FilterMode::Linear,
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
