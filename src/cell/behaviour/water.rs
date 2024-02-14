use rand::Rng;

use crate::{cell::Cell, world::World};

use super::CellBehaviour;

#[derive(Clone, Copy)]
pub struct WaterBehaviour {
    pub dispersion_rate: isize,
    pub density: f32,
}

impl CellBehaviour for WaterBehaviour {
    fn next_position(&mut self, x: usize, y: usize, world: &World) -> (usize, usize) {
        let strategy: &dyn Fn(&Cell, isize, isize) -> bool = &|cell, _, _| {
            if cell.id.is_air() {
                return true;
            }

            if let Some(cell_density) = cell.behaviour.get_density() {
                if cell_density < self.density {
                    return true;
                }
            }

            false
        };

        let (new_x, new_y) = self.check_cells_along_line_strat(
            x as isize,
            y as isize,
            world,
            x as isize,
            y as isize + self.dispersion_rate,
            strategy,
        );

        if (new_x, new_y) != (x, y) {
            return (new_x, new_y);
        }

        let dir = if rand::thread_rng().gen() { 1 } else { -1 };

        let (new_x, new_y) = self.check_cells_along_line_strat(
            x as isize,
            y as isize,
            world,
            x as isize + (self.dispersion_rate * dir),
            y as isize,
            strategy,
        );

        if (new_x, new_y) != (x, y) {
            return (new_x, new_y);
        }

        let (new_x, new_y) = self.check_cells_along_line_strat(
            x as isize,
            y as isize,
            world,
            x as isize + (self.dispersion_rate * -dir),
            y as isize,
            strategy,
        );

        if (new_x, new_y) != (x, y) {
            return (new_x, new_y);
        }

        for i in 0..=self.dispersion_rate as usize {
            let dx = x as isize + (i as isize * dir);
            let (new_x, new_y) = self.check_cells_along_line_strat(
                x as isize,
                y as isize,
                world,
                dx,
                (y as isize + self.dispersion_rate) as isize,
                strategy,
            );

            if (new_x, new_y) != (x, y) {
                return (new_x, new_y);
            }
        }

        for i in 0..=self.dispersion_rate as usize {
            let dx = x as isize + (i as isize * -dir);
            let (new_x, new_y) = self.check_cells_along_line_strat(
                x as isize,
                y as isize,
                world,
                dx,
                (y as isize + self.dispersion_rate) as isize,
                strategy,
            );

            if (new_x, new_y) != (x, y) {
                return (new_x, new_y);
            }
        }

        (x, y)
    }

    fn get_density(&self) -> Option<f32> {
        Some(self.density)
    }

    fn clone_box(&self) -> Box<dyn CellBehaviour> {
        Box::new(*self)
    }
}
