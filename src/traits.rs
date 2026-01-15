use sdl3::pixels::Color;

use crate::cell::Position;

pub trait GetColor {
    fn get_color(&self) -> Color;
}

pub trait GetPosition {
    fn get_position(&self) -> Position;
}

pub trait Behaviour {
    fn update(&mut self);
}
