#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellId {
    Air = 0,
    Sand = 1,
}

impl CellId {
    pub fn base_color(&self) -> [u8; 4] {
        match self {
            CellId::Air => [0, 0, 0, 0],
            CellId::Sand => [255, 255, 255, 255],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub id: CellId,
    pub color: [u8; 4],
}

impl Cell {
    pub const AIR: Cell = Cell {
        id: CellId::Air,
        color: [0, 0, 0, 0],
    };
}
