use fltk::{app, prelude::*,button::*, frame::*, group::*,  window};

const GRID_SIZE: i32 = 9;

fn main() {
    let app = app::App::default();

    let mut wind = window::Window::new(100, 100, 600, 600, "Empty Window");

    let mut grid = Pack::new(10, 10, 380, 380, "");
    grid.make_resizable(true);

    // Create a 9x9 grid of buttons
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            grid.end();
            let mut button = Button::new(30 + col * 40, 30 + row * 40, 40, 40, "1");
        }
    }
    grid.end();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}
