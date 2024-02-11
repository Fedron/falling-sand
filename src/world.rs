use crate::cell::{Cell, CellId};

pub struct World {
    cells: Vec<Cell>,
}

impl World {
    const WIDTH: usize = 64;
    const HEIGHT: usize = 48;

    pub fn new() -> Self {
        let mut cells = Vec::with_capacity(Self::WIDTH * Self::HEIGHT);
        for _ in 0..Self::WIDTH * Self::HEIGHT {
            cells.push(Cell::new(CellId::Air));
        }

        Self { cells }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(Self::coord_to_index(x, y))
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.cells.get_mut(Self::coord_to_index(x, y))
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if x >= Self::WIDTH || y >= Self::HEIGHT {
            return;
        }

        self.cells[Self::coord_to_index(x, y)] = cell;
    }

    pub fn update(&mut self) {
        for x in 0..Self::WIDTH {
            for y in 0..Self::HEIGHT {
                if let Some(mut cell) = self.get_cell(x, y).cloned() {
                    if cell.moved_this_frame {
                        continue;
                    }

                    let (new_x, new_y) = cell.next_position(x, y, self);

                    if (new_x, new_y) != (x, y) {
                        if let Some(next_cell) =
                            self.cells.get(Self::coord_to_index(new_x, new_y)).cloned()
                        {
                            self.set_cell(x, y, next_cell);
                        }
                    }
                    self.set_cell(new_x, new_y, cell);
                }
            }
        }
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % Self::WIDTH;
            let y = i / Self::WIDTH;

            let cell = &mut self.cells[Self::coord_to_index(x, y)];
            cell.moved_this_frame = false;
            pixel.copy_from_slice(&cell.color);
        }
    }

    fn coord_to_index(x: usize, y: usize) -> usize {
        (y * World::WIDTH).saturating_add(x)
    }
}
