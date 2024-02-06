use rand::Rng;

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
            cells: [Cell::AIR; Self::WIDTH * Self::HEIGHT],
            next_cells: [Cell::AIR; Self::WIDTH * Self::HEIGHT],
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
        self.next_cells = [Cell::AIR; Self::WIDTH * Self::HEIGHT];
        for x in 0..Self::WIDTH {
            for y in 0..Self::HEIGHT {
                let cell = self.get_cell(x, y);
                if cell.is_none() {
                    continue;
                }
                let cell = cell.unwrap();

                match cell.id {
                    CellId::Air => {}
                    CellId::Sand => {
                        if let Some(below) = self.get_cell(x, y.saturating_add(1)) {
                            if below.id == CellId::Air {
                                self.next_cells[Self::coord_to_index(x, y.saturating_add(1))] =
                                    cell.clone();
                                continue;
                            }
                        }

                        let dir = if rand::thread_rng().gen_bool(0.5) {
                            -1
                        } else {
                            1
                        };

                        if let Some(below_a) =
                            self.get_cell(x.saturating_add_signed(dir), y.saturating_add(1))
                        {
                            if below_a.id == CellId::Air {
                                self.next_cells[Self::coord_to_index(
                                    x.saturating_add_signed(dir),
                                    y.saturating_add(1),
                                )] = cell.clone();
                                continue;
                            }
                        }

                        if let Some(below_b) =
                            self.get_cell(x.saturating_add_signed(dir), y.saturating_add(1))
                        {
                            if below_b.id == CellId::Air {
                                self.next_cells[Self::coord_to_index(
                                    x.saturating_add_signed(dir),
                                    y.saturating_add(1),
                                )] = cell.clone();
                                continue;
                            }
                        }

                        self.next_cells[Self::coord_to_index(x, y)] = cell.clone();
                    }
                    CellId::Stone => {
                        self.next_cells[Self::coord_to_index(x, y)] = cell.clone();
                    }
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
        y * World::WIDTH + x
    }
}
