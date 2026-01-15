pub trait GetColor {
    fn get_color(&self) -> (u8, u8, u8);
}

pub trait GetPosition {
    fn get_position(&self) -> (usize, usize);
}
