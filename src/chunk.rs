use crate::cell::{Cell, CellType};

pub const CHUNK_WIDTH: usize = 64;
pub const CHUNK_HEIGHT: usize = 64;

pub struct Chunk {
    pub cells: [[Option<Cell>; CHUNK_HEIGHT]; CHUNK_WIDTH],
}

impl Chunk {
    pub fn new() -> Self {
        let mut cells = [[None; CHUNK_HEIGHT]; CHUNK_WIDTH];
        cells[CHUNK_WIDTH / 2][CHUNK_HEIGHT / 2] = Some(Cell {
            cell_type: CellType::Sand,
            last_update: 0,
        });

        Self { cells }
    }

    pub fn update(&mut self, update_counter: usize) {
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                if let Some(cell) = self.cells[x][y] {
                    if cell.last_update == update_counter {
                        continue;
                    }

                    let new_position = cell.move_cell(self, x, y);
                    if new_position != (x, y) {
                        self.cells[new_position.0][new_position.1]
                            .as_mut()
                            .unwrap()
                            .last_update = update_counter;
                    }
                }
            }
        }
    }

    pub fn draw(&self, texture: &mut [u8]) {
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                let start = (x * 4 + y * CHUNK_WIDTH * 4) as usize;
                if let Some(cell) = self.cells[x][y] {
                    let color = match cell.cell_type {
                        CellType::Sand => [255, 0, 0, 255],
                    };
                    texture[start..start + 4].copy_from_slice(&color);
                } else {
                    texture[start..start + 4].copy_from_slice(&[0, 0, 0, 0]);
                }
            }
        }
    }
}
