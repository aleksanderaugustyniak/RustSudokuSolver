use fltk::{app, prelude::*,button::*, enums::*, frame::*, group::*,  window};

const GRID_SIZE: usize = 9;

pub struct Grid {}

impl Grid {
    pub fn new() -> Self {
        Grid {}
    }

    pub fn display(&mut self) {
        let app = app::App::default();

        let mut wind = window::Window::new(100, 100, 600, 600, "Empty Window");

        let mut grid = Pack::new(10, 10, 380, 380, "");
        grid.make_resizable(true);

        let mut buttons: [[Button; GRID_SIZE]; GRID_SIZE] = Default::default();

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                grid.end();
                let mut button = Button::new(30 + col as i32 * 40, 30 + row as i32 * 40, 40, 40, "1");
                buttons[row][col] = button;
            }
        }
        grid.end();

        wind.make_resizable(true);
        wind.end();
        wind.show();

        app.run().unwrap();
    }
}