use crate::{cell::Position, traits::GetPosition};

#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<Option<()>>>,
}

impl Grid {
    pub fn new(size: (usize, usize)) -> Self {
        Self {
            cells: vec![vec![None; size.1]; size.0],
        }
    }

    pub fn add<T: GetPosition>(&mut self, object: &T) {
        let (x, y) = object.get_position();
        if self.cells[x][y].is_none() {
            self.cells[x][y] = Some(());
        }
    }

    pub fn del<T: GetPosition>(&mut self, object: &T) {
        let (x, y) = object.get_position();
        if self.cells[x][y].is_some() {
            self.cells[x][y] = None;
        }
    }

    pub fn is_empty(&self, pos: Position) -> bool {
        self.cells[pos.0][pos.1].is_none()
    }
}
