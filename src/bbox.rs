pub struct BoundingBox {
    pub min: cgmath::Point2<f32>,
    pub max: cgmath::Point2<f32>,
}

impl BoundingBox {
    pub fn contains(&self, position: cgmath::Point2<f32>) -> bool {
        position.x >= self.min.x
            && position.x <= self.max.x
            && position.y >= self.min.y
            && position.y <= self.max.y
    }
}
