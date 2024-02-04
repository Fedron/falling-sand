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
        let mut cells = [Cell::Air; Self::WIDTH * Self::HEIGHT];
        cells[Self::coord_to_index(32, 0)] = Cell::Sand;

        Self {
            cells,
            next_cells: [Cell::Air; Self::WIDTH * Self::HEIGHT],
        }
    }

    pub fn update(&mut self) {
        self.next_cells = [Cell::Air; Self::WIDTH * Self::HEIGHT];
        for x in 0..Self::WIDTH {
            for y in 0..Self::HEIGHT {
                let cell = self.cells[Self::coord_to_index(x, y)];
                match cell {
                    Cell::Air => {}
                    Cell::Sand => {
                        if y < Self::HEIGHT - 1 {
                            let below = self.cells[Self::coord_to_index(x, y + 1)];
                            if below == Cell::Air {
                                self.next_cells[Self::coord_to_index(x, y + 1)] = Cell::Sand;
                            }
                            continue;
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
