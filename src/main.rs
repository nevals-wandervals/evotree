extern crate sdl3;

use evotree::cell::Cell;
use evotree::grid::Grid;
use evotree::traits::{GetColor, GetPosition};
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;
use std::time::Duration;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Evotree", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let cells = vec![Cell::new((250, 250))];

    let mut grid = Grid::new((500, 500));
    grid.add(&cells[0]).unwrap();

    let mut canvas = window.into_canvas();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(25, 25, 30));
        canvas.clear();

        render(&mut canvas, &cells);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn render<T: GetColor + GetPosition>(canvas: &mut Canvas<Window>, objs: &Vec<T>) {
    for obj in objs.iter() {
        let (x, y) = obj.get_position();
        canvas.set_draw_color(obj.get_color());
        canvas
            .fill_rect(Rect::new(x as i32, y as i32, 1, 1))
            .unwrap();
    }
}
