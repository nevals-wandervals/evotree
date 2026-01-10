use crate::cell::Cell;

#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<Option<Cell>>>,
}

impl Grid {
    pub fn new(size: (usize, usize)) -> Self {
        Self {
            cells: vec![vec![None; size.1]; size.0],
        }
    }
}
