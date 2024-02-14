use bresenham::Bresenham;

use crate::world::World;

use super::Cell;

pub mod air;
pub mod sand;
pub mod solid;
pub mod water;

pub trait CellBehaviour {
    fn next_position(&mut self, x: usize, y: usize, world: &World) -> (usize, usize);
    fn get_density(&self) -> Option<f32>;

    fn clone_box(&self) -> Box<dyn CellBehaviour>;

    fn check_cells_along_line_strat(
        &self,
        start_x: isize,
        start_y: isize,
        world: &World,
        end_x: isize,
        end_y: isize,
        strategy: &dyn Fn(&Cell, isize, isize) -> bool,
    ) -> (usize, usize) {
        let mut new_x = start_x;
        let mut new_y = start_y;

        for (x, y) in Bresenham::new((start_x, start_y), (end_x, end_y)) {
            if x == start_x && y == start_y {
                continue;
            }

            if let Some(cell) = world.get_cell(x as usize, y as usize) {
                if strategy(cell, x, y) {
                    new_x = x;
                    new_y = y;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        (new_x as usize, new_y as usize)
    }

    fn check_cells_along_line(
        &self,
        start_x: isize,
        start_y: isize,
        world: &World,
        end_x: isize,
        end_y: isize,
    ) -> (usize, usize) {
        self.check_cells_along_line_strat(start_x, start_y, world, end_x, end_y, &|cell, _, _| {
            cell.id.is_air()
        })
    }
}

impl Clone for Box<dyn CellBehaviour> {
    fn clone(&self) -> Box<dyn CellBehaviour> {
        self.clone_box()
    }
}
