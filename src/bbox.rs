use cgmath::ElementWise;

pub struct BoundingBox {
    pub min: cgmath::Point2<f32>,
    pub max: cgmath::Point2<f32>,
}

impl BoundingBox {
    pub fn from_center(
        center: cgmath::Point2<f32>,
        min: cgmath::Point2<f32>,
        max: cgmath::Point2<f32>,
    ) -> Self {
        let half_width = (max.x - min.x) / 2.0;
        let half_height = (max.y - min.y) / 2.0;
        let half_extents = cgmath::Point2::new(half_width, half_height);

        Self {
            min: center.sub_element_wise(half_extents),
            max: center.add_element_wise(half_extents),
        }
    }

    pub fn contains(&self, position: cgmath::Point2<f32>) -> bool {
        position.x >= self.min.x
            && position.x <= self.max.x
            && position.y >= self.min.y
            && position.y <= self.max.y
    }
}
