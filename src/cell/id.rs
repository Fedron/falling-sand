use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellId {
    Air = 0,
    Sand = 1,
    Stone = 2,
    Water = 3,
    Dirt = 4,
    Coal = 5,
}

impl From<u8> for CellId {
    fn from(item: u8) -> Self {
        match item {
            0 => CellId::Air,
            1 => CellId::Sand,
            2 => CellId::Stone,
            3 => CellId::Water,
            4 => CellId::Dirt,
            5 => CellId::Coal,
            _ => panic!("Invalid cell id: {}", item),
        }
    }
}

impl CellId {
    const COLOR_VARIANCE: u8 = 0x05;

    pub fn base_color(&self) -> [u8; 4] {
        match self {
            CellId::Air => [0, 0, 0, 0],
            CellId::Sand => [0xff, 0xf4, 0x9f, 0xff],
            CellId::Stone => [0x80, 0x80, 0x80, 0xff],
            CellId::Water => [0x57, 0xa4, 0xff, 0xff],
            CellId::Dirt => [0x92, 0x61, 0x18, 0xff],
            CellId::Coal => [0x53, 0x53, 0x53, 0xff],
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

    pub fn is_air(&self) -> bool {
        match self {
            CellId::Air => true,
            _ => false,
        }
    }

    pub fn is_solid(&self) -> bool {
        match self {
            CellId::Air => false,
            CellId::Sand => true,
            CellId::Stone => true,
            CellId::Water => false,
            CellId::Dirt => true,
            CellId::Coal => true,
        }
    }
}
