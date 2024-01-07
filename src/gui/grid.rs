use fltk::{app, prelude::*, button::*, enums::*, group::*, window};
use std::cell::RefCell;
use std::rc::Rc;

const GRID_SIZE: usize = 9;
const BUTTON_SIZE: i32 = 50;
const GRID_OFFSET_FROM_LEFT: i32 = 30;

pub struct Grid {
    current_number: Rc<RefCell<String>>,
    control_panel: Rc<RefCell<[Button; GRID_SIZE]>>,
}

impl Grid{
    pub fn new() -> Self {
        Grid {
            current_number: Rc::new(RefCell::new("1".to_string())),
            control_panel: Default::default(),
        }
    }

    pub fn display(&mut self) {
        let app = app::App::default();
        let mut window = window::Window::new(100, 100, 600, 600, "Sudoku");
        window.set_color(Color::White);
        let mut grid = Pack::new(
            10,
            10,
            BUTTON_SIZE * GRID_SIZE as i32,
            BUTTON_SIZE * GRID_SIZE as i32,
            "");
            grid.make_resizable(true);
            
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                grid.end();
                self.display_button(row, col);
            }
        }
        grid.end();
        self.display_control_panel();

        window.make_resizable(true);
        window.end();
        window.show();

        app.run().unwrap();
    }

    fn display_control_panel(&mut self) {
        for number in 0..9 {
            self.control_panel.borrow_mut()[number] = Button::new(
                GRID_OFFSET_FROM_LEFT + (number) as i32 * BUTTON_SIZE,
                BUTTON_SIZE * GRID_SIZE as i32 + 50,
                BUTTON_SIZE,
                BUTTON_SIZE,
                "1",
            );
            self.control_panel.borrow_mut()[number].set_label(&format!("{}", number + 1));
        }
        for number in 0..9 {
            let current_number_clone = Rc::clone(&self.current_number);
            let control_panel_clone = Rc::clone(&self.control_panel);

            self.control_panel.borrow_mut()[number].set_callback(move |button: &mut Button| {
                *current_number_clone.borrow_mut() = button.label().to_string();
                for number in 0..9 {
                    (*control_panel_clone.borrow_mut())[number].set_color(Color::White.darker());
                    (*control_panel_clone.borrow_mut())[number].redraw();
                }
                button.set_color(Color::Blue.lighter());
            });
        }
    }

    fn display_button(&self, row: usize, col: usize) {
        let mut button = Button::new(
            GRID_OFFSET_FROM_LEFT + col as i32 * BUTTON_SIZE,
            30 + row as i32 * BUTTON_SIZE,
            BUTTON_SIZE,
            BUTTON_SIZE,
            "1",
        );
        button.set_label("");

        let button_label = Rc::clone(&self.current_number);
            button.set_callback(move |button: &mut Button| {
            button.set_label(&format!("{}", button_label.borrow()));
        });

        let square_id = (row / 3) + (col / 3);
        if square_id % 2 == 1 {
            button.set_color(button.color().lighter());
        }
    }
}
