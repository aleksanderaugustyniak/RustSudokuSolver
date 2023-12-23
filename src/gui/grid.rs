use fltk::{app, prelude::*,button::*, enums::*, frame::*, group::*,  window};

const GRID_SIZE: usize = 9;
const BUTTON_SIZE: i32 = 50;
const GRID_OFFSET_FROM_LEFT: i32 = 30;

pub struct Grid {
    sudoku_grid : [[Button; GRID_SIZE]; GRID_SIZE],
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            sudoku_grid : Default::default(),
        }
    }

    pub fn display(&mut self) {
        let app = app::App::default();

        let mut wind = window::Window::new(100, 100, 600, 600, "Sudoku");

        let mut grid = Pack::new(10, 10, BUTTON_SIZE*GRID_SIZE as i32, BUTTON_SIZE*GRID_SIZE as i32, "");
        grid.make_resizable(true);

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                grid.end();
                let mut button = Button::new(GRID_OFFSET_FROM_LEFT + col as i32 * BUTTON_SIZE, 30 + row as i32 * BUTTON_SIZE, BUTTON_SIZE, BUTTON_SIZE, "1");
                self.sudoku_grid[row][col] = button;
            }
        }
        grid.end();
        
        for number in 1..10 {
            let mut button = Button::new(GRID_OFFSET_FROM_LEFT + (number - 1) as i32 * BUTTON_SIZE, BUTTON_SIZE*GRID_SIZE as i32 + 50, BUTTON_SIZE, BUTTON_SIZE, "1");
        }
        

        wind.make_resizable(true);
        wind.end();
        wind.show();

        app.run().unwrap();
    }
}