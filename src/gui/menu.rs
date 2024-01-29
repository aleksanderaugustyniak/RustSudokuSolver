use fltk::{ menu, prelude::* };
use std::cell::RefCell;
use std::rc::Rc;
use crate::gui::play_board::PlayBoard;

const MENU_WIDTH: i32 = 25;

pub struct Menu {
    _menu_bar: menu::MenuBar,
    file_menu: menu::MenuButton,
    solve_menu: menu::MenuButton,
    board: Rc<RefCell<PlayBoard>>,
}

impl Menu {
    pub fn new(play_board: Rc<RefCell<PlayBoard>>, window_width: i32) -> Self {
        Menu {
            _menu_bar: menu::MenuBar::new(0, 0, window_width, MENU_WIDTH, ""),
            file_menu: menu::MenuButton::new(0, 0, 60, MENU_WIDTH, "Board"),
            solve_menu: menu::MenuButton::new(60, 0, 60, MENU_WIDTH, "Solve"),
            board: Rc::clone(&play_board),
        }
    }

    pub fn display(&mut self) {
        self.file_menu.add_choice("Save");
        self.file_menu.add_choice("Read");
        self.file_menu.add_choice("Clear");
        self.set_file_callback();
        self.solve_menu.add_choice("Solve");
        self.solve_menu.add_choice("Show notes");
        self.set_solve_callback();
    }

    fn set_file_callback(&mut self) {
        let file_menu_clone = self.file_menu.clone();
        let board_clone = Rc::clone(&self.board);

        self.file_menu.set_callback(move |_| {
            match file_menu_clone.value() {
                0 => {
                    if let Err(err) = board_clone.borrow().to_json() {
                        eprintln!("Error writing to JSON file: {}", err);
                    }
                }
                1 => {
                    if let Err(err) = board_clone.borrow_mut().read_from_file() {
                        eprintln!("Error updating from JSON file: {}", err);
                    }
                }
                2 => {
                    (*board_clone.borrow_mut()).clear();
                }
                _ => {}
            }
        });
    }

    fn set_solve_callback(&mut self) {
        let solve_menu_clone = self.solve_menu.clone();
        let board_clone = Rc::clone(&self.board);

        self.solve_menu.set_callback(move |_| {
            match solve_menu_clone.value() {
                0 => {
                    board_clone.borrow_mut().solve_puzzle();
                }
                1 => {
                    board_clone.borrow_mut().show_notes();
                }
                _ => {}
            }
        });
    }
}
