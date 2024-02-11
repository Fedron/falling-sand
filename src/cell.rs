use bresenham::Bresenham;
use rand::Rng;

use crate::world::World;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellId {
    Air = 0,
    Sand = 1,
    Stone = 2,
    Water = 3,
    Dirt = 4,
    Coal = 5,
}

impl From<u8> for CellId {
    fn from(item: u8) -> Self {
        match item {
            0 => CellId::Air,
            1 => CellId::Sand,
            2 => CellId::Stone,
            3 => CellId::Water,
            4 => CellId::Dirt,
            5 => CellId::Coal,
            _ => panic!("Invalid cell id: {}", item),
        }
    }
}

impl CellId {
    const COLOR_VARIANCE: u8 = 0x05;

    pub fn base_color(&self) -> [u8; 4] {
        match self {
            CellId::Air => [0, 0, 0, 0],
            CellId::Sand => [0xff, 0xf4, 0x9f, 0xff],
            CellId::Stone => [0x80, 0x80, 0x80, 0xff],
            CellId::Water => [0x57, 0xa4, 0xff, 0xff],
            CellId::Dirt => [0x92, 0x61, 0x18, 0xff],
            CellId::Coal => [0x53, 0x53, 0x53, 0xff],
        }
    }

    pub fn varied_color(&self) -> [u8; 4] {
        match self {
            CellId::Air => [0, 0, 0, 0],
            _ => {
                let base_color = self.base_color();
                let mut rng = rand::thread_rng();
                let r = rng.gen_range(
                    base_color[0].saturating_sub(Self::COLOR_VARIANCE)
                        ..=base_color[0].saturating_add(Self::COLOR_VARIANCE),
                );
                let g = rng.gen_range(
                    base_color[1].saturating_sub(Self::COLOR_VARIANCE)
                        ..=base_color[1].saturating_add(Self::COLOR_VARIANCE),
                );
                let b = rng.gen_range(
                    base_color[2].saturating_sub(Self::COLOR_VARIANCE)
                        ..=base_color[2].saturating_add(Self::COLOR_VARIANCE),
                );
                [r, g, b, base_color[3]]
            }
        }
    }

    pub fn is_air(&self) -> bool {
        match self {
            CellId::Air => true,
            _ => false,
        }
    }

    pub fn is_solid(&self) -> bool {
        match self {
            CellId::Air => false,
            CellId::Sand => true,
            CellId::Stone => true,
            CellId::Water => false,
            CellId::Dirt => true,
            CellId::Coal => true,
        }
    }
}

pub trait CellBehaviour {
    fn next_position(&mut self, x: usize, y: usize, world: &World) -> (usize, usize);
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

pub struct AirBehaviour;
pub struct SolidBehaviour;

#[derive(Clone, Copy)]
pub struct SandBehaviour {
    velocity_y: f32,
    velocity_x: f32,
    collision_velocity_loss: f32,
    friction: f32,
}

#[derive(Clone, Copy)]
pub struct WaterBehaviour {
    dispersion_rate: isize,
}

impl CellBehaviour for AirBehaviour {
    fn next_position(&mut self, x: usize, y: usize, _world: &World) -> (usize, usize) {
        (x, y)
    }

    fn clone_box(&self) -> Box<dyn CellBehaviour> {
        Box::new(AirBehaviour)
    }
}

impl CellBehaviour for SolidBehaviour {
    fn next_position(&mut self, x: usize, y: usize, _world: &World) -> (usize, usize) {
        (x, y)
    }

    fn clone_box(&self) -> Box<dyn CellBehaviour> {
        Box::new(SolidBehaviour)
    }
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

#[derive(Clone)]
pub struct Cell {
    pub id: CellId,
    pub color: [u8; 4],
    pub moved_this_frame: bool,
    pub behaviour: Box<dyn CellBehaviour>,

    pub is_stationary: bool,
    same_pos_count: usize,
}

impl Cell {
    pub fn new(id: CellId) -> Self {
        Self {
            id,
            color: id.varied_color(),
            moved_this_frame: false,
            behaviour: match id {
                CellId::Air => Box::new(AirBehaviour),
                CellId::Sand => Box::new(SandBehaviour {
                    velocity_y: 1.0,
                    velocity_x: 0.0,
                    collision_velocity_loss: 1.5,
                    friction: 0.9,
                }),
                CellId::Stone => Box::new(SolidBehaviour),
                CellId::Water => Box::new(WaterBehaviour { dispersion_rate: 2 }),
                CellId::Dirt => Box::new(SandBehaviour {
                    velocity_y: 1.0,
                    velocity_x: 0.0,
                    collision_velocity_loss: 1.7,
                    friction: 0.8,
                }),
                CellId::Coal => Box::new(SandBehaviour {
                    velocity_y: 1.0,
                    velocity_x: 0.0,
                    collision_velocity_loss: 2.0,
                    friction: 0.7,
                }),
            },

            is_stationary: true,
            same_pos_count: 0,
        }
    }

    pub fn is_solid(&self) -> bool {
        self.id.is_solid()
    }

    pub fn next_position(&mut self, x: usize, y: usize, world: &World) -> (usize, usize) {
        let (new_x, new_y) = self.behaviour.next_position(x, y, world);

        if (new_x, new_y) != (x, y) {
            self.moved_this_frame = true;
            self.is_stationary = false;
            self.same_pos_count = 0;
        } else {
            self.same_pos_count += 1;
        }

        if self.same_pos_count > 1 {
            self.is_stationary = true;
        }

        (new_x, new_y)
    }
}
