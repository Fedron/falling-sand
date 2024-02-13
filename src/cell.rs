use crate::world::World;

use self::{behaviour::{air::AirBehaviour, sand::SandBehaviour, solid::SolidBehaviour, water::WaterBehaviour, CellBehaviour}, id::CellId};

pub mod behaviour;
pub mod id;

#[derive(Clone)]
pub struct Cell {
    pub id: CellId,
    pub color: [u8; 4],
    pub moved_this_frame: bool,
    pub behaviour: Box<dyn CellBehaviour>,

    pub is_stationary: bool,
    same_pos_count: usize,
}

impl Cell {
    pub fn new(id: CellId) -> Self {
        Self {
            id,
            color: id.varied_color(),
            moved_this_frame: false,
            behaviour: match id {
                CellId::Air => Box::new(AirBehaviour),
                CellId::Sand => Box::new(SandBehaviour {
                    velocity_y: 1.0,
                    velocity_x: 0.0,
                    collision_velocity_loss: 1.5,
                    friction: 0.9,
                }),
                CellId::Stone => Box::new(SolidBehaviour),
                CellId::Water => Box::new(WaterBehaviour { dispersion_rate: 2 }),
                CellId::Dirt => Box::new(SandBehaviour {
                    velocity_y: 1.0,
                    velocity_x: 0.0,
                    collision_velocity_loss: 1.7,
                    friction: 0.8,
                }),
                CellId::Coal => Box::new(SandBehaviour {
                    velocity_y: 1.0,
                    velocity_x: 0.0,
                    collision_velocity_loss: 2.0,
                    friction: 0.7,
                }),
            },

            is_stationary: true,
            same_pos_count: 0,
        }
    }

    pub fn is_solid(&self) -> bool {
        self.id.is_solid()
    }

    pub fn next_position(&mut self, x: usize, y: usize, world: &World) -> (usize, usize) {
        let (new_x, new_y) = self.behaviour.next_position(x, y, world);

        if (new_x, new_y) != (x, y) {
            self.moved_this_frame = true;
            self.is_stationary = false;
            self.same_pos_count = 0;
        } else {
            if !self.is_stationary {
                self.same_pos_count += 1;
            }
        }

        if self.same_pos_count > 1 {
            self.is_stationary = true;
        }

        (new_x, new_y)
    }
}
