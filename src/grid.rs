use std::collections::HashMap;

use crate::cell::Cell;

#[derive(Debug)]
pub struct Grid {
    pub alive_cells: HashMap<(usize, usize), ()>,
    pub cells: Vec<Vec<Option<Cell>>>,
}

impl Grid {
    pub fn new(size: (usize, usize)) -> Self {
        Self {
            alive_cells: HashMap::new(),
            cells: vec![vec![None; size.1]; size.0],
        }
    }

    pub fn add(&mut self, x: usize, y: usize, cell: Cell) {
        if !self.alive_cells.contains_key(&(x, y)) {
            self.alive_cells.insert((x, y), ());
            self.cells[x][y] = Some(cell);
        }
    }

    pub fn add_replace(&mut self, x: usize, y: usize, cell: Cell) {
        if !self.alive_cells.contains_key(&(x, y)) {
            self.alive_cells.insert((x, y), ());
        }

        self.cells[x][y] = Some(cell);
    }

    pub fn del(&mut self, x: usize, y: usize) {
        self.alive_cells.remove(&(x, y));
        self.cells[x][y] = None;
    }

    pub fn get_pos_alive_cells(&self) -> Vec<&(usize, usize)> {
        self.alive_cells.keys().collect()
    }
}
