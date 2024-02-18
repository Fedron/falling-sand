use crate::chunk::{Chunk, CHUNK_HEIGHT};

#[derive(Clone, Copy)]
pub struct Cell {
    pub cell_type: CellType,
    pub last_update: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Sand,
}

impl Cell {
    pub fn move_cell(&self, chunk: &mut Chunk, x: usize, y: usize) -> (usize, usize) {
        if y < CHUNK_HEIGHT - 1 {
            if chunk.cells[x][y + 1].is_none() {
                chunk.cells[x][y + 1] = chunk.cells[x][y].take();
                return (x, y + 1);
            }
        }

        (x, y)
    }
}
