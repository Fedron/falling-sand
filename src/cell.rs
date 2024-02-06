use rand::Rng;

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
