use std::ops::{Index, IndexMut};

use sdl3::pixels::Color;

use crate::traits::GetColor;

#[derive(Debug, Clone, Copy)]
pub struct Genome {
    step: usize,
    genes: [Gene; 32],
}

impl Genome {
    pub fn new() -> Self {
        Self {
            step: 0,
            genes: [Gene::default(); 32],
        }
    }

    pub fn next_step(&mut self) {
        self.step += 1;
    }

    pub fn reset(&mut self) {
        self.step = 0;
    }

    pub fn get_gene(&self) -> &Gene {
        &self.genes[self.step]
    }

    pub fn get_mut_gene(&mut self) -> &mut Gene {
        &mut self.genes[self.step]
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Gene(pub [TypeCell; 5]);

impl Index<Direction> for Gene {
    type Output = TypeCell;
    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Direction::Center => &self.0[0],
            Direction::Left => &self.0[1],
            Direction::Top => &self.0[2],
            Direction::Right => &self.0[3],
            Direction::Bottom => &self.0[4],
        }
    }
}

impl IndexMut<Direction> for Gene {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        match index {
            Direction::Center => &mut self.0[0],
            Direction::Left => &mut self.0[1],
            Direction::Top => &mut self.0[2],
            Direction::Right => &mut self.0[3],
            Direction::Bottom => &mut self.0[4],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Center,
    Left,
    Top,
    Right,
    Bottom,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum TypeCell {
    Seed,
    Root,
    Trunk,
    Builder,
    Leaf,
    #[default]
    None,
}

impl GetColor for TypeCell {
    /// color seed: #736e64ff\
    /// color root: #8a473bff\
    /// color trunk: #b1b56bff\
    /// color builder: #42611dff\
    /// color leaf: #659b4eff
    fn get_color(&self) -> Color {
        match self {
            TypeCell::Seed => Color::RGB(0x73, 0x6e, 0x64),
            TypeCell::Root => Color::RGB(0x8a, 0x47, 0x3b),
            TypeCell::Trunk => Color::RGB(0xb1, 0xb5, 0x6b),
            TypeCell::Builder => Color::RGB(0x42, 0x61, 0x1d),
            TypeCell::Leaf => Color::RGB(0x65, 0x9b, 0x4e),
            TypeCell::None => Color::RGB(0x00, 0x00, 0x00),
        }
    }
}
