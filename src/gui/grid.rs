use fltk::{app, prelude::*, button::*, enums::*, group::*, window};
use std::cell::RefCell;
use std::rc::Rc;

const GRID_SIZE: usize = 9;
const BUTTON_SIZE: i32 = 50;
const GRID_OFFSET_FROM_LEFT: i32 = 30;
const LIGHT_BUTTON_COLOR: Color = Color::from_rgb(200, 200, 200);
const DARK_BUTTON_COLOR: Color = Color::from_rgb(150, 150, 150);
const HIGHLIGHTED_BUTTON_COLOR: Color = Color::from_rgb(100, 100, 250);

pub struct Grid {
    current_number: Rc<RefCell<String>>,
    control_panel: Rc<RefCell<[Button; GRID_SIZE]>>,
    play_grid: Rc<RefCell<[[Button; GRID_SIZE]; GRID_SIZE]>>,
}

impl Grid{
    pub fn new() -> Self {
        Grid {
            current_number: Rc::new(RefCell::new("1".to_string())),
            control_panel: Default::default(),
            play_grid: Default::default(),
        }
    }

    pub fn display(&mut self) {
        let app = app::App::default();
        let mut window = window::Window::new(100, 100, 600, 600, "Sudoku");
        window.set_color(Color::White);
        
        self.display_play_grid(); 
        self.display_control_panel();

        window.make_resizable(true);
        window.end();
        window.show();

        app.run().unwrap();
    }

    fn display_play_grid(&mut self) {
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
    }

    fn display_control_panel(&mut self) {
        for number in 0..9 {
            self.control_panel.borrow_mut()[number] = Button::new(
                GRID_OFFSET_FROM_LEFT + (number) as i32 * BUTTON_SIZE,
                BUTTON_SIZE * GRID_SIZE as i32 + 50,
                BUTTON_SIZE,
                BUTTON_SIZE,
                "",
            );
            self.control_panel.borrow_mut()[number].set_label(&format!("{}", number + 1));
            self.control_panel.borrow_mut()[number].set_color(LIGHT_BUTTON_COLOR);
        }
        self.set_control_panel_callbacks();
    }

    fn set_control_panel_callbacks(&mut self) {
        for control_button in self.control_panel.borrow_mut().iter_mut() {
            let current_number_clone = Rc::clone(&self.current_number);
            let control_panel_clone = Rc::clone(&self.control_panel);

            control_button.set_callback(move |button: &mut Button| {
                *current_number_clone.borrow_mut() = button.label().to_string();
                for but in (*control_panel_clone.borrow_mut()).iter_mut() {
                    but.set_color(LIGHT_BUTTON_COLOR);
                    but.redraw();
                }
                button.set_color(HIGHLIGHTED_BUTTON_COLOR);
            });
        }
    }

    fn display_button(&self, row: usize, col: usize) {
        self.play_grid.borrow_mut()[row][col] = Button::new(
            GRID_OFFSET_FROM_LEFT + col as i32 * BUTTON_SIZE,
            30 + row as i32 * BUTTON_SIZE,
            BUTTON_SIZE,
            BUTTON_SIZE,
            "",
        );

        self.set_play_button_callback(row, col);
        self.show_squares(row, col);
    }

    fn set_play_button_callback(&self, row: usize, col: usize) {
        let button_label = Rc::clone(&self.current_number);
        self.play_grid.borrow_mut()[row][col].set_callback(move |button: &mut Button| {
            button.set_label(&format!("{}", button_label.borrow()));
        });
    }

    fn show_squares(&self, row: usize, col: usize) {
        let square_id = (row / 3) + (col / 3);
        let button_color = if square_id % 2 == 1 {DARK_BUTTON_COLOR} else {LIGHT_BUTTON_COLOR};
        self.play_grid.borrow_mut()[row][col].set_color(button_color);
    }
}
