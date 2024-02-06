#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Air = 0,
    Sand = 1,
}

pub struct World {
    cells: [Cell; World::WIDTH * World::HEIGHT],
    next_cells: [Cell; World::WIDTH * World::HEIGHT],
}

impl World {
    const WIDTH: usize = 64;
    const HEIGHT: usize = 48;

    pub fn new() -> Self {
        Self {
            cells: [Cell::Air; Self::WIDTH * Self::HEIGHT],
            next_cells: [Cell::Air; Self::WIDTH * Self::HEIGHT],
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        if x >= Self::WIDTH || y >= Self::HEIGHT {
            return None;
        }

        Some(self.cells[Self::coord_to_index(x, y)])
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if x >= Self::WIDTH || y >= Self::HEIGHT {
            return;
        }

        self.cells[Self::coord_to_index(x, y)] = cell;
    }

    pub fn update(&mut self) {
        self.next_cells = [Cell::Air; Self::WIDTH * Self::HEIGHT];
        for x in 0..Self::WIDTH {
            for y in 0..Self::HEIGHT {
                let cell = self.get_cell(x, y);
                if cell.is_none() {
                    continue;
                }
                let cell = cell.unwrap();

                match cell {
                    Cell::Air => {}
                    Cell::Sand => {
                        if let Some(below) = self.get_cell(x, y.saturating_add(1)) {
                            if below == Cell::Air {
                                self.next_cells[Self::coord_to_index(x, y.saturating_add(1))] =
                                    Cell::Sand;
                                continue;
                            }
                        }

                        if let Some(below_right) =
                            self.get_cell(x.saturating_add(1), y.saturating_add(1))
                        {
                            if below_right == Cell::Air {
                                self.next_cells[Self::coord_to_index(
                                    x.saturating_add(1),
                                    y.saturating_add(1),
                                )] = Cell::Sand;
                                continue;
                            }
                        }

                        if let Some(below_right) =
                            self.get_cell(x.saturating_sub(1), y.saturating_add(1))
                        {
                            if below_right == Cell::Air {
                                self.next_cells[Self::coord_to_index(
                                    x.saturating_sub(1),
                                    y.saturating_add(1),
                                )] = Cell::Sand;
                                continue;
                            }
                        }

                        self.next_cells[Self::coord_to_index(x, y)] = Cell::Sand;
                    }
                }
            }
        }
        self.cells = self.next_cells;
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % Self::WIDTH;
            let y = i / Self::WIDTH;

            let cell = self.cells[Self::coord_to_index(x, y)];
            pixel.copy_from_slice(match cell {
                Cell::Air => &[0, 0, 0, 0],
                Cell::Sand => &[255, 255, 255, 255],
            });
        }
    }

    fn coord_to_index(x: usize, y: usize) -> usize {
        y * World::WIDTH + x
    }
}
