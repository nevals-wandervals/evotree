use sdl3::pixels::Color;

use crate::{
    genome::{Genome, TypeCell},
    traits::{GetColor, GetPosition},
};

pub type Position = (usize, usize);

#[derive(Debug, Clone)]
pub struct Cell {
    pub position: Position,
    pub type_cell: TypeCell,
    pub energy: f32,
    pub lifetime: usize,
    pub genome: Genome,
}

impl Cell {
    pub fn new(pos: Position) -> Self {
        Self {
            position: pos,
            type_cell: TypeCell::Seed,
            energy: 8.0,
            lifetime: 0,
            genome: Genome::new(),
        }
    }
}

impl GetPosition for Cell {
    fn get_position(&self) -> Position {
        self.position
    }
}

impl GetColor for Cell {
    fn get_color(&self) -> Color {
        self.type_cell.get_color()
    }
}
