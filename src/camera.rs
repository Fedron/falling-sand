use cgmath::EuclideanSpace;

pub struct Camera {
    pub position: cgmath::Point3<f32>,
    projection: cgmath::Matrix4<f32>,
    width: f32,
    height: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            position: cgmath::Point3::new(0.0, 0.0, 0.0),
            projection: cgmath::ortho(0.0, width, 0.0, height, -1.0, 1.0),
            width,
            height,
        }
    }

    pub fn update_size(&mut self, width: f32, height: f32) {
        self.projection = cgmath::ortho(0.0, width, 0.0, height, -1.0, 1.0);
        self.width = width;
        self.height = height;
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::from_translation(-self.position.to_vec());
        self.projection * view
    }

    pub fn window_pos_to_world_pos(&self, window_pos: cgmath::Point2<f32>) -> cgmath::Point2<f32> {
        let x = window_pos.x + self.position.x;
        let y = (self.height - window_pos.y) + self.position.y;
        cgmath::Point2::new(x, y)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_projection(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into()
    }
}
