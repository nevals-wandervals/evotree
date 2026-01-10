use crate::{
    genome::{Genome, TypeCell},
    traits::GetColor,
};

#[derive(Debug, Clone)]
pub struct Cell {
    pub type_cell: TypeCell,
    pub energy: f32,
    pub lifetime: usize,
    pub genome: Genome,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            type_cell: TypeCell::Seed,
            energy: 8.0,
            lifetime: 0,
            genome: Genome::new(),
        }
    }
}

impl GetColor for Cell {
    fn get_color(&self) -> (u8, u8, u8) {
        self.type_cell.get_color()
    }
}
