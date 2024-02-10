use rand::Rng;

use crate::world::World;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellId {
    Air = 0,
    Sand = 1,
    Stone = 2,
}

impl CellId {
    const COLOR_VARIANCE: u8 = 0x05;

    pub fn base_color(&self) -> [u8; 4] {
        match self {
            CellId::Air => [0, 0, 0, 0],
            CellId::Sand => [0xff, 0xf4, 0x9f, 0xff],
            CellId::Stone => [0x80, 0x80, 0x80, 0xff],
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

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub id: CellId,
    pub color: [u8; 4],
    pub velocity: f32,
    pub moved: bool,
}

impl Cell {
    pub fn new(id: CellId) -> Self {
        Self {
            id,
            color: id.varied_color(),
            velocity: 0.0,
            moved: false,
        }
    }

    pub fn next_position(&self, x: usize, y: usize, world: &World) -> (usize, usize) {
        match self.id {
            CellId::Air => (x, y),
            CellId::Sand => {
                let mut found_move = false;
                let mut new_y = y;
                for _ in 0..self.velocity as usize {
                    if let Some(below) = world.get_cell(x, new_y + 1) {
                        if below.id != CellId::Air {
                            break;
                        }
                        found_move = true;
                        new_y += 1;
                    }
                }

                if found_move {
                    return (x, new_y);
                }

                let dir = if rand::thread_rng().gen_bool(0.5) {
                    1
                } else {
                    -1
                };

                let mut found_move = false;
                let mut new_y = y;
                for _ in 0..self.velocity as usize {
                    if let Some(below) = world.get_cell(x.saturating_add_signed(dir), new_y + 1) {
                        if below.id != CellId::Air {
                            break;
                        }
                        found_move = true;
                        new_y += 1;
                    }
                }

                if found_move {
                    return (x.saturating_add_signed(dir), new_y);
                }

                let mut found_move = false;
                let mut new_y = y;
                for _ in 0..self.velocity as usize {
                    if let Some(below) = world.get_cell(x.saturating_add_signed(-dir), new_y + 1) {
                        if below.id != CellId::Air {
                            break;
                        }
                        found_move = true;
                        new_y += 1;
                    }
                }

                if found_move {
                    return (x.saturating_add_signed(-dir), new_y);
                }

                (x, y)
            }
            CellId::Stone => (x, y),
        }
    }
}
