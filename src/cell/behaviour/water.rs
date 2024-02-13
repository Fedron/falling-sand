use rand::Rng;

use crate::world::World;

use super::CellBehaviour;

#[derive(Clone, Copy)]
pub struct WaterBehaviour {
    pub dispersion_rate: isize,
}

impl CellBehaviour for WaterBehaviour {
    fn next_position(&mut self, x: usize, y: usize, world: &World) -> (usize, usize) {
        let (new_x, new_y) = self.check_cells_along_line(
            x as isize,
            y as isize,
            world,
            x as isize,
            y as isize + self.dispersion_rate,
        );

        if (new_x, new_y) != (x, y) {
            return (new_x, new_y);
        }

        let dir = if rand::thread_rng().gen() { 1 } else { -1 };

        let (new_x, new_y) = self.check_cells_along_line(
            x as isize,
            y as isize,
            world,
            x as isize + (self.dispersion_rate * dir),
            y as isize,
        );

        if (new_x, new_y) != (x, y) {
            return (new_x, new_y);
        }

        let (new_x, new_y) = self.check_cells_along_line(
            x as isize,
            y as isize,
            world,
            x as isize + (self.dispersion_rate * -dir),
            y as isize,
        );

        if (new_x, new_y) != (x, y) {
            return (new_x, new_y);
        }

        for i in 0..=self.dispersion_rate as usize {
            let dx = x as isize + (i as isize * dir);
            let (new_x, new_y) = self.check_cells_along_line(
                x as isize,
                y as isize,
                world,
                dx,
                (y as isize + self.dispersion_rate) as isize,
            );

            if (new_x, new_y) != (x, y) {
                return (new_x, new_y);
            }
        }

        for i in 0..=self.dispersion_rate as usize {
            let dx = x as isize + (i as isize * -dir);
            let (new_x, new_y) = self.check_cells_along_line(
                x as isize,
                y as isize,
                world,
                dx,
                (y as isize + self.dispersion_rate) as isize,
            );

            if (new_x, new_y) != (x, y) {
                return (new_x, new_y);
            }
        }

        (x, y)
    }

    fn clone_box(&self) -> Box<dyn CellBehaviour> {
        Box::new(*self)
    }
}
