use fltk::{ prelude::*, button::*, group::*, enums::* };
use std::cell::RefCell;
use std::rc::Rc;
use crate::gui::board::*;
use crate::gui::colors::*;
use crate::gui::save_handler::Saver;

const MENU_WIDTH: i32 = 25;
const BUTTON_SIZE: i32 = 50;
const BOARD_OFFSET_LEFT: i32 = 30;
const BOARD_OFFSET_TOP: i32 = 2 * MENU_WIDTH;

pub struct PlayBoard {
    play_grid: Rc<RefCell<Board>>,
    current_number: Rc<RefCell<String>>,
}

impl PlayBoard {
    pub fn new() -> Self {
        PlayBoard {
            play_grid: Default::default(),
            current_number: Rc::new(RefCell::new("1".to_string())),
        }
    }

    pub fn display(&mut self) {
        let mut grid = Pack::new(
            10,
            10,
            BUTTON_SIZE * (GRID_SIZE as i32),
            BUTTON_SIZE * (GRID_SIZE as i32),
            ""
        );
        grid.make_resizable(true);

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                grid.end();
                self.display_button(row, col);
            }
        }
        self.clear_color();
        grid.end();
    }

    pub fn set_number(&mut self, number: &str) {
        *self.current_number.borrow_mut() = String::from(number);
    }

    fn display_button(&self, row: usize, col: usize) {
        self.play_grid.borrow_mut()[row][col] = Button::new(
            BOARD_OFFSET_LEFT + (col as i32) * BUTTON_SIZE,
            BOARD_OFFSET_TOP + (row as i32) * BUTTON_SIZE,
            BUTTON_SIZE,
            BUTTON_SIZE,
            ""
        );

        self.set_callback(row, col);
    }

    fn set_callback(&self, row: usize, col: usize) {
        let button_label = Rc::clone(&self.current_number);
        self.play_grid.borrow_mut()[row][col].set_callback(move |button: &mut Button| {
            button.set_label(&format!("{}", button_label.borrow()));
            button.set_color(HIGHLIGHTED_BUTTON_COLOR);
        });
    }

    pub fn clear_color(&mut self) {
        for (row, play_row) in self.play_grid.borrow_mut().iter_mut().enumerate() {
            for (col, button) in play_row.iter_mut().enumerate() {
                button.set_color(Self::get_square_color(row, col));
                button.redraw();
            }
        }
    }

    pub fn clear(&mut self) {
        for play_row in self.play_grid.borrow_mut().iter_mut() {
            for button in play_row.iter_mut() {
                button.set_label("");
            }
        }
        self.clear_color();
    }

    fn get_square_color(row: usize, col: usize) -> Color {
        let square_id = row / 3 + col / 3;
        if square_id % 2 == 1 {
            DARK_BUTTON_COLOR
        } else {
            LIGHT_BUTTON_COLOR
        }
    }

    pub fn to_json(&self) -> Result<(), Box<dyn std::error::Error>> {
        Saver::to_json("boards/board.json", &self.play_grid.borrow())
    }

    pub fn from_json(&self) -> Result<(), Box<dyn std::error::Error>> {
        Saver::from_json("boards/board.json", &mut *self.play_grid.borrow_mut())
    }

    pub fn highlight(&mut self, label: &str) {
        for (row, play_row) in self.play_grid.borrow_mut().iter_mut().enumerate() {
            for (col, button) in play_row.iter_mut().enumerate() {
                let color = if button.label() == label {
                    HIGHLIGHTED_BUTTON_COLOR
                } else {
                    Self::get_square_color(row, col)
                };
                button.set_color(color);
                button.redraw();
            }
        }
    }
}
