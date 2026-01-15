use crate::{cell::Position, traits::GetPosition};

#[derive(Debug)]
pub struct Grid {
    pub size: (usize, usize),
    pub cells: Vec<Vec<Option<()>>>,
}

impl Grid {
    pub fn new(size: (usize, usize)) -> Self {
        Self {
            size,
            cells: vec![vec![None; size.1]; size.0],
        }
    }

    pub fn add<T: GetPosition>(&mut self, object: &T) -> Result<Position, ()> {
        let pos = self.check_bounds(object)?;
        if self.is_empty(pos) {
            self.cells[pos.0][pos.1] = Some(());
        }

        Ok(pos)
    }

    pub fn del<T: GetPosition>(&mut self, object: &T) -> Result<Position, ()> {
        let pos = self.check_bounds(object)?;
        if !self.is_empty(pos) {
            self.cells[pos.0][pos.1] = None;
        }

        Ok(pos)
    }

    pub fn is_empty(&self, pos: Position) -> bool {
        self.cells[pos.0][pos.1].is_none()
    }

    fn check_bounds<T: GetPosition>(&self, object: &T) -> Result<Position, ()> {
        self::check_bounds(((1, self.size.0), (1, self.size.1)), object)
    }
}

pub fn check_bounds<T: GetPosition>(
    ((min_x, max_x), (min_y, max_y)): ((usize, usize), (usize, usize)),
    pos: &T,
) -> Result<Position, ()> {
    let (x, y) = pos.get_position();
    if x < min_x || y < min_y || x >= max_x || y >= max_y {
        return Err(());
    }

    Ok((x, y))
}
