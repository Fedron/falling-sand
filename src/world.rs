use crate::cell::{Cell, CellId};

pub struct World {
    cells: [Cell; World::WIDTH * World::HEIGHT],
    next_cells: [Cell; World::WIDTH * World::HEIGHT],
}

impl World {
    const WIDTH: usize = 64;
    const HEIGHT: usize = 48;

    pub fn new() -> Self {
        Self {
            cells: [Cell::new(CellId::Air); Self::WIDTH * Self::HEIGHT],
            next_cells: [Cell::new(CellId::Air); Self::WIDTH * Self::HEIGHT],
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        if x >= Self::WIDTH || y >= Self::HEIGHT {
            return None;
        }

        Some(self.cells[Self::coord_to_index(x, y)].clone())
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if x >= Self::WIDTH || y >= Self::HEIGHT {
            return;
        }

        self.cells[Self::coord_to_index(x, y)] = cell;
    }

    pub fn update(&mut self) {
        self.next_cells = [Cell::new(CellId::Air); Self::WIDTH * Self::HEIGHT];

        for x in 0..Self::WIDTH {
            for y in 0..Self::HEIGHT {
                if let Some(mut cell) = self.get_cell(x, y) {
                    if cell.id == CellId::Air {
                        continue;
                    }

                    let (new_x, new_y) = cell.next_position(x, y, &self);
                    cell.velocity += 0.1;
                    self.next_cells[Self::coord_to_index(new_x, new_y)] = cell;
                }
            }
        }

        self.cells = self.next_cells.clone();
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % Self::WIDTH;
            let y = i / Self::WIDTH;

            let cell = &self.cells[Self::coord_to_index(x, y)];
            pixel.copy_from_slice(&cell.color);
        }
    }

    fn coord_to_index(x: usize, y: usize) -> usize {
        (y * World::WIDTH + x).clamp(0, World::WIDTH * World::HEIGHT - 1)
    }
}
