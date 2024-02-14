use crate::world::World;

use super::CellBehaviour;

pub struct SolidBehaviour;

impl CellBehaviour for SolidBehaviour {
    fn next_position(&mut self, x: usize, y: usize, _world: &World) -> (usize, usize) {
        (x, y)
    }

    fn get_density(&self) -> Option<f32> {
        None
    }

    fn clone_box(&self) -> Box<dyn CellBehaviour> {
        Box::new(SolidBehaviour)
    }
}
