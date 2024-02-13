use rand::Rng;

use crate::world::World;

use super::CellBehaviour;

#[derive(Clone, Copy)]
pub struct SandBehaviour {
    pub velocity_y: f32,
    pub velocity_x: f32,
    pub collision_velocity_loss: f32,
    pub friction: f32,
}

impl CellBehaviour for SandBehaviour {
    fn next_position(&mut self, x: usize, y: usize, world: &World) -> (usize, usize) {
        if self.velocity_x.abs() > 0.95 {
            self.velocity_x *= self.friction;
            if self.velocity_x.abs() < 0.1 {
                self.velocity_x = 0.0;
            }
        }

        self.velocity_y += 0.1;

        let (new_x, new_y) = self.check_cells_along_line(
            x as isize,
            y as isize,
            world,
            x as isize,
            (y as f32 + self.velocity_y) as isize,
        );

        let dir = if rand::thread_rng().gen() { 1 } else { -1 };

        if (new_x, new_y) != (x, y) {
            if let Some(cell) = world.get_cell(new_x, new_y + 1) {
                if cell.id.is_solid() && cell.is_stationary {
                    let abs_y = self.velocity_y.abs() / self.collision_velocity_loss;
                    self.velocity_x = abs_y * dir as f32;
                }
            }

            return (new_x, new_y);
        }

        for i in 0..=self.velocity_y as usize {
            let dx = x as isize + (i as isize * dir) + self.velocity_x as isize;
            let (new_x, new_y) = self.check_cells_along_line(
                x as isize,
                y as isize,
                world,
                dx,
                (y as f32 + self.velocity_y) as isize,
            );

            if (new_x, new_y) != (x, y) {
                return (new_x, new_y);
            }
        }

        for i in 0..=self.velocity_y as usize {
            let dx = x as isize + (i as isize * -dir) + self.velocity_x as isize;
            let (new_x, new_y) = self.check_cells_along_line(
                x as isize,
                y as isize,
                world,
                dx,
                (y as f32 + self.velocity_y) as isize,
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
