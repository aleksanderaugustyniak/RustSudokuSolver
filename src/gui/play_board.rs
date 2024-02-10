use fltk::{ prelude::*, button::Button, group::Pack };
use fltk_theme::widget_themes;
use std::cell::RefCell;
use std::rc::Rc;
use crate::common::grid_size::GRID_SIZE;
use crate::common::puzzle::Puzzle;
use crate::gui::board::Board;
use crate::gui::consts::*;
use crate::gui::save_handler::*;
use crate::solve::solver::Solver;

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
        self.clear_highlight();
        grid.end();
    }

    pub fn set_number(&mut self, number: &str) {
        *self.current_number.borrow_mut() = String::from(number);
    }

    fn display_button(&self, row: usize, col: usize) {
        let square_spacing = 5;
        let square_x = (col as i32) / 3;
        let square_y = (row as i32) / 3;
        self.play_grid.borrow_mut()[row][col] = Button::new(
            BOARD_OFFSET_LEFT + (col as i32) * BUTTON_SIZE + square_spacing * square_x,
            BOARD_OFFSET_TOP + (row as i32) * BUTTON_SIZE + square_spacing * square_y,
            BUTTON_SIZE,
            BUTTON_SIZE,
            ""
        );
        self.play_grid.borrow_mut()[row][col].set_label_size(16);
        self.play_grid
            .borrow_mut()
            [row][col].set_label_color(fltk::enums::Color::from_rgb(0, 0, 0));
        self.set_callback(row, col);
    }

    fn set_callback(&self, row: usize, col: usize) {
        let button_label = Rc::clone(&self.current_number);
        self.play_grid.borrow_mut()[row][col].set_callback(move |button: &mut Button| {
            button.set_label(&format!("{}", button_label.borrow()));
            button.set_label_size(16);
            button.set_label_color(fltk::enums::Color::from_rgb(0, 0, 0));
            button.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
        });
    }

    pub fn clear_highlight(&mut self) {
        for play_row in self.play_grid.borrow_mut().iter_mut() {
            for button in play_row.iter_mut() {
                button.set_frame(widget_themes::OS_BUTTON_UP_BOX);
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
        self.clear_highlight();
    }

    pub fn to_json(&self) -> Result<(), Box<dyn std::error::Error>> {
        to_json("boards/board.json", &self.play_grid.borrow())
    }

    pub fn read_from_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        from_json("boards/board.json", &mut self.play_grid.borrow_mut())
    }

    pub fn solve_puzzle(&mut self) {
        let mut solver = Solver::new(crate::gui::adapter::read_puzzle(&self.play_grid.borrow()));
        solver.solve();
        self.display_puzzle(&solver.get_solution());
    }

    pub fn show_notes(&mut self) {
        let mut notes_manager = crate::solve::notes_manager::NotesManager::new(
            crate::gui::adapter::read_puzzle(&self.play_grid.borrow())
        );
        notes_manager.fill();
        let notes = notes_manager.get();
        for (row, x) in notes.iter().enumerate() {
            for (col, note) in x.iter().enumerate() {
                if notes[row][col] != 0 {
                    let button = &mut self.play_grid.borrow_mut()[row][col];
                    button.set_label(&Self::note_to_string(*note).to_string());
                    button.set_label_size(10);
                    button.set_label_color(fltk::enums::Color::from_rgb(80, 80, 240));
                    button.redraw();
                }
            }
        }
    }

    fn note_to_string(note: u16) -> String {
        let mut output = String::new();
        for i in 0..GRID_SIZE {
            if (note & (1 << i)) != 0 {
                output.push_str(&(i + 1).to_string());
            } else {
                output.push(' ');
            }
            if i == 2 || i == 5 {
                output.push('\n');
            }
        }
        output
    }

    fn display_puzzle(&mut self, solution: &Puzzle) {
        for (row, sol_row) in solution.iter().enumerate() {
            for (col, cell) in sol_row.iter().enumerate() {
                self.play_grid.borrow_mut()[row][col].set_label(&cell.to_string());
            }
        }
    }

    pub fn highlight(&mut self, label: &str) {
        for play_row in self.play_grid.borrow_mut().iter_mut() {
            for button in play_row.iter_mut() {
                if button.label() == label {
                    button.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
                } else {
                    button.set_frame(widget_themes::OS_BUTTON_UP_BOX);
                }
                button.redraw();
            }
        }
    }
}
