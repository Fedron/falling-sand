use bresenham::Bresenham;
use rand::Rng;

use crate::world::World;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellId {
    Air = 0,
    Sand = 1,
    Stone = 2,
    Water = 3,
}

impl From<u8> for CellId {
    fn from(item: u8) -> Self {
        match item {
            0 => CellId::Air,
            1 => CellId::Sand,
            2 => CellId::Stone,
            3 => CellId::Water,
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
}

pub trait CellBehaviour {
    fn next_position(&mut self, x: usize, y: usize, world: &World) -> (usize, usize);
    fn clone_box(&self) -> Box<dyn CellBehaviour>;

    fn check_cells_along_line(
        &self,
        start_x: isize,
        start_y: isize,
        world: &World,
        end_x: isize,
        end_y: isize,
    ) -> (usize, usize) {
        let mut new_x = start_x;
        let mut new_y = start_y;

        for (x, y) in Bresenham::new((start_x, start_y), (end_x, end_y)) {
            if x == start_x && y == start_y {
                continue;
            }

            if let Some(cell) = world.get_cell(x as usize, y as usize) {
                if cell.id != CellId::Air {
                    break;
                }

                new_x = x;
                new_y = y;
            } else {
                break;
            }
        }

        (new_x as usize, new_y as usize)
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
    velocity: f32,
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
        self.velocity += 0.1;

        let (new_x, new_y) = self.check_cells_along_line(
            x as isize,
            y as isize,
            world,
            x as isize,
            (y as f32 + self.velocity) as isize,
        );

        if (new_x, new_y) != (x, y) {
            return (new_x, new_y);
        }

        let dir = if rand::thread_rng().gen() { 1 } else { -1 };

        for i in 0..=self.velocity as usize {
            let dx = x as isize + (i as isize * dir);
            let (new_x, new_y) = self.check_cells_along_line(
                x as isize,
                y as isize,
                world,
                dx,
                (y as f32 + self.velocity) as isize,
            );

            if (new_x, new_y) != (x, y) {
                return (new_x, new_y);
            }
        }

        for i in 0..=self.velocity as usize {
            let dx = x as isize + (i as isize * -dir);
            let (new_x, new_y) = self.check_cells_along_line(
                x as isize,
                y as isize,
                world,
                dx,
                (y as f32 + self.velocity) as isize,
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
    pub moved: bool,
    pub behaviour: Box<dyn CellBehaviour>,
}

impl Cell {
    pub fn new(id: CellId) -> Self {
        Self {
            id,
            color: id.varied_color(),
            moved: false,
            behaviour: match id {
                CellId::Air => Box::new(AirBehaviour),
                CellId::Sand => Box::new(SandBehaviour { velocity: 1.0 }),
                CellId::Stone => Box::new(SolidBehaviour),
                CellId::Water => Box::new(WaterBehaviour { dispersion_rate: 2 }),
            },
        }
    }

    pub fn next_position(&mut self, x: usize, y: usize, world: &World) -> (usize, usize) {
        let (new_x, new_y) = self.behaviour.next_position(x, y, world);

        if (new_x, new_y) != (x, y) {
            self.moved = true;
        }

        (new_x, new_y)
    }
}
